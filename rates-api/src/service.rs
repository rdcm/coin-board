use crate::domain::GetRatesQueryHandler;
use crate::domain_impl::GetRatesQueryHandlerImpl;
use crate::endpoints::get_rates;
use crate::infrastructure::RatesRepositoryImpl;
use crate::settings::Settings;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use mongodb::options::ClientOptions;
use mongodb::Client as MongoClient;
use std::sync::Arc;
use actix_cors::Cors;

pub struct Service {
    server: Server,
}

impl Service {
    pub async fn build(settings: &Settings) -> Self {
        let client_options = ClientOptions::parse(&settings.database.uri).await.unwrap();
        let client = MongoClient::with_options(client_options).unwrap();

        let rates_repository =
            Arc::new(RatesRepositoryImpl::new(client, &settings.database.db_name));
        let handler: Arc<dyn GetRatesQueryHandler> =
            Arc::new(GetRatesQueryHandlerImpl::new(rates_repository.clone()));

        let addr = settings.endpoints.get_api_address();
        let http_server = HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .max_age(3600),
                )
                .service(get_rates)
                .app_data(Data::from(handler.clone()))
        })
        .bind(&addr)
        .unwrap_or_else(|_| panic!("Failed to bind to the host: {}", &addr));

        let server = http_server.run();

        Self { server }
    }

    pub async fn run(self) -> Option<()> {
        self.server.await.ok()
    }
}
