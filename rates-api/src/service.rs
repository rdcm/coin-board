use crate::domain::GetRatesQueryHandler;
use crate::domain_impl::GetRatesQueryHandlerImpl;
use crate::endpoints::get_rates;
use crate::infrastructure::RatesRepositoryImpl;
use crate::metrics::{get_metrics_handler, Metrics};
use crate::settings::Settings;
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{dev::Server, dev::Service as DevService, web, App, HttpServer};
use anyhow::{Context, Result};
use mongodb::options::ClientOptions;
use mongodb::Client as MongoClient;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

pub struct Service {
    api_server: Server,
    metrics_server: Server,
}

impl Service {
    pub async fn build(settings: Settings) -> Result<Self> {
        let metrics = Arc::new(Metrics::new()?);

        let client_options = ClientOptions::parse(&settings.database.uri)
            .await
            .context("[rates-api] [mongodb] Failed parse database URI")?;

        let client = MongoClient::with_options(client_options)
            .context("[rates-api] [mongodb] Failed create mongodb client")?;

        let rates_repository = Arc::new(RatesRepositoryImpl::new(
            client,
            &settings.database.db_name,
        ));

        let handler: Arc<dyn GetRatesQueryHandler> = Arc::new(GetRatesQueryHandlerImpl::new(
            rates_repository.clone(),
        ));

        let api_addr = settings.endpoints.get_api_address();
        let api_server = HttpServer::new({
            let metrics = metrics.clone();
            move || {
                App::new()
                    .wrap(TracingLogger::default())
                    .wrap(
                        Cors::default()
                            .allowed_origin(&settings.cors.origin)
                            .allowed_methods(settings.cors.methods.split(' '))
                            .allowed_headers(settings.cors.headers.split(' '))
                            .max_age(settings.cors.max_age),
                    )
                    .app_data(Data::new(metrics.clone()))
                    .wrap_fn(|req, srv| {
                        let metrics = req.app_data::<Data<Arc<Metrics>>>().unwrap().clone();
                        let timer = metrics.request_duration.start_timer();
                        metrics.request_count.inc();

                        let fut = srv.call(req);

                        async move {
                            let res = fut.await?;
                            timer.observe_duration();
                            Ok(res)
                        }
                    })
                    .service(get_rates)
                    .app_data(Data::from(handler.clone()))
            }
        })
        .bind(&api_addr)
        .context(format!(
            "[rates-api] Failed to bind api server to the host: {}",
            &api_addr
        ))?
        .run();

        let metrics_addr = settings.metrics.get_api_address();
        let metrics_server = HttpServer::new({
            let metrics = metrics.clone();
            move || {
                App::new()
                    .wrap(TracingLogger::default())
                    .app_data(Data::new(metrics.clone()))
                    .route("/metrics", web::get().to(get_metrics_handler))
            }
        })
        .bind(&metrics_addr)
        .context(format!(
            "[rates-api] Failed to bind metrics server to the host: {}",
            &metrics_addr
        ))?
        .run();

        Ok(Self {
            api_server,
            metrics_server,
        })
    }

    pub async fn run(self) -> Result<()> {
        let (_, _) = tokio::try_join!(self.api_server, self.metrics_server)
            .map_err(|e| anyhow::anyhow!("[rates-api] Failed to run servers: {:?}", e))?;
        Ok(())
    }
}
