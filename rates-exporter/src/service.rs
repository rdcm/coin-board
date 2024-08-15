use crate::domain::{FetchDataQuery, FetchRatesQueryHandler};
use crate::domain_impl::FetchRatesQueryHandlerImpl;
use crate::infrastructure::{RatesProviderImpl, RatesRepositoryImpl};
use crate::settings::Settings;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::Client;
use std::sync::Arc;

pub struct Service {
    handler: FetchRatesQueryHandlerImpl,
    settings: Settings,
}

impl Service {
    pub async fn build(settings: Settings) -> Self {
        let mut client_options = ClientOptions::parse(&settings.database.uri).await.unwrap();
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options).unwrap();

        let repository = RatesRepositoryImpl::new(client, &settings.database.db_name);
        let rates_provider = RatesProviderImpl::new(settings.provider.uri.to_string());
        let handler =
            FetchRatesQueryHandlerImpl::new(Arc::new(repository), Arc::new(rates_provider));

        Service { handler, settings }
    }

    pub async fn run(&self) -> Option<()> {
        let query = FetchDataQuery::new(
            self.settings.provider.fetch_delay_per_page_ms,
            self.settings.provider.page_size,
        );
        self.handler.handle(&query).await
    }
}
