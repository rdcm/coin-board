use crate::domain::{FetchDataQuery, FetchRatesQueryHandler};
use crate::domain_impl::FetchRatesQueryHandlerImpl;
use crate::infrastructure::{BinanceRatesProvider, RatesRepositoryImpl};
use crate::settings::Settings;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::Client;
use std::sync::Arc;

pub struct Service {
    handler: FetchRatesQueryHandlerImpl,
}

impl Service {
    pub async fn build(settings: &Settings) -> Self {
        let mut client_options = ClientOptions::parse(&settings.db_settings.uri)
            .await
            .unwrap();
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options).unwrap();

        let repository = RatesRepositoryImpl::new(client, &settings.db_settings.db_name);
        let rates_provider = BinanceRatesProvider::new(settings.provider_settings.uri.to_string());
        let handler = FetchRatesQueryHandlerImpl::new(Arc::new(repository), Arc::new(rates_provider));

        Service { handler }
    }

    pub async fn run(&self) -> Option<()> {
        let query = FetchDataQuery::new("Binance".to_string());
        self.handler.handle(&query).await
    }
}
