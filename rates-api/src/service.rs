use crate::domain::GetRatesQueryHandler;
use crate::domain_impl::GetRatesQueryHandlerImpl;
use crate::endpoints::get_rates;
use crate::infrastructure::RatesRepositoryImpl;
use crate::settings::Settings;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::Client as MongoClient;
use std::sync::Arc;

pub struct Service {
    server: Server,
}

impl Service {
    pub async fn build(settings: &Settings) -> Self {
        let mut client_options = ClientOptions::parse(&settings.database.uri).await.unwrap();
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = MongoClient::with_options(client_options).unwrap();

        let rates_repository =
            Arc::new(RatesRepositoryImpl::new(client, &settings.database.db_name));
        let handler: Arc<dyn GetRatesQueryHandler> =
            Arc::new(GetRatesQueryHandlerImpl::new(rates_repository.clone()));

        let addr = settings.endpoints.get_api_address();
        let http_server = HttpServer::new(move || {
            App::new()
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
