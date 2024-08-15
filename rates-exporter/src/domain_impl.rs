use crate::domain::{FetchDataQuery, FetchRatesQueryHandler};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct FetchRatesQueryHandlerImpl {
    pub repository: Arc<dyn RatesRepository + Send + Sync>,
    pub rates_provider: Arc<dyn RatesProvider + Send + Sync>,
}

#[async_trait::async_trait]
impl FetchRatesQueryHandler for FetchRatesQueryHandlerImpl {
    async fn handle(&self, query: &FetchDataQuery) -> Option<()> {
        let rates = self.rates_provider.get_rates().await?;
        self.repository
            .insert(query.source.to_string(), rates)
            .await?;
        Some(())
    }
}

impl FetchRatesQueryHandlerImpl {
    pub fn new(
        repository: Arc<dyn RatesRepository + Send + Sync>,
        rates_provider: Arc<dyn RatesProvider + Send + Sync>,
    ) -> Self {
        FetchRatesQueryHandlerImpl {
            repository,
            rates_provider,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CurrencyRate {
    pub symbol: String,
    pub price: String,
}

#[async_trait::async_trait]
pub trait RatesRepository {
    async fn insert(&self, source: String, rates: Vec<CurrencyRate>) -> Option<()>;
}

#[async_trait::async_trait]
pub trait RatesProvider {
    async fn get_rates(&self) -> Option<Vec<CurrencyRate>>;
}
