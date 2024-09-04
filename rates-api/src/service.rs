use crate::domain::GetRatesQueryHandler;
use crate::domain_impl::GetRatesQueryHandlerImpl;
use crate::endpoints::get_rates;
use crate::infrastructure::RatesRepositoryImpl;
use crate::metrics::{metrics_handler, Metrics};
use crate::settings::Settings;
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{dev::Server, dev::Service as DevService, web, App, HttpServer};
use mongodb::options::ClientOptions;
use mongodb::Client as MongoClient;
use std::sync::Arc;

pub struct Service {
    api_server: Server,
    metrics_server: Server,
}

impl Service {
    pub async fn build(settings: Settings) -> Self {
        let metrics = Arc::new(Metrics::new());
        let app_metrics = Arc::clone(&metrics);

        let client_options = ClientOptions::parse(&settings.database.uri).await.unwrap();
        let client = MongoClient::with_options(client_options).unwrap();

        let rates_repository =
            Arc::new(RatesRepositoryImpl::new(client, &settings.database.db_name));
        let handler: Arc<dyn GetRatesQueryHandler> =
            Arc::new(GetRatesQueryHandlerImpl::new(rates_repository.clone()));

        let api_addr = settings.endpoints.get_api_address();
        let api_server = HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::default()
                        .allowed_origin(&settings.cors.origin)
                        .allowed_methods(settings.cors.methods.split(' '))
                        .allowed_headers(settings.cors.headers.split(' '))
                        .max_age(settings.cors.max_age),
                )
                .app_data(web::Data::new(app_metrics.clone()))
                .wrap_fn(|req, srv| {
                    let metrics = req.app_data::<web::Data<Arc<Metrics>>>().unwrap().clone();
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
        })
        .bind(&api_addr)
        .unwrap_or_else(|_| panic!("Failed to bind api server to the host: {}", &api_addr))
        .run();

        let metrics_addr = settings.metrics.get_api_address();
        let metrics_server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(metrics.clone()))
                .route("/metrics", web::get().to(metrics_handler))
        })
        .bind(&metrics_addr)
        .unwrap_or_else(|_| panic!("Failed to bind metrics server to the host: {}", &api_addr))
        .run();

        Self {
            api_server,
            metrics_server,
        }
    }

    pub async fn run(self) -> Option<()> {
        tokio::join!(self.api_server, self.metrics_server);
        Some(())
    }
}
