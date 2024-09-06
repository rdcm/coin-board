use crate::domain::{FetchDataQuery, FetchRatesQueryHandler};
use crate::domain_impl::FetchRatesQueryHandlerImpl;
use crate::infrastructure::{RatesProviderImpl, RatesRepositoryImpl};
use crate::settings::Settings;
use anyhow::{Context, Result};
use mongodb::options::ClientOptions;
use mongodb::Client as MongoClient;
use std::sync::Arc;

pub struct Service {
    handler: FetchRatesQueryHandlerImpl,
    settings: Settings,
}

impl Service {
    pub async fn build(settings: Settings) -> Result<Self> {
        let client_options = ClientOptions::parse(&settings.database.uri)
            .await
            .context("[rates-exporter] [mongodb] Failed parse database URI")?;

        let client = MongoClient::with_options(client_options)
            .context("[rates-exporter] [mongodb] Failed create mongodb client")?;

        let repository = RatesRepositoryImpl::new(client, &settings.database.db_name);
        let rates_provider = RatesProviderImpl::new(settings.provider.uri.to_string());
        let handler =
            FetchRatesQueryHandlerImpl::new(Arc::new(repository), Arc::new(rates_provider));

        Ok(Service { handler, settings })
    }

    pub async fn run(&self) -> Result<()> {
        let query = FetchDataQuery {
            coins_ids: self.settings.provider.coins.clone(),
        };

        self.handler.handle(&query).await?;

        Ok(())
    }
}
