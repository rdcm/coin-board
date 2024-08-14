use crate::domain::{FetchDataQuery, Handler};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct FetchDataQueryHandler {
    pub repository: Arc<dyn IRatesRepository + Send + Sync>,
    pub rates_provider: Arc<dyn RatesProvider + Send + Sync>,
}

#[async_trait::async_trait]
impl Handler for FetchDataQueryHandler {
    async fn handle(&self, query: &FetchDataQuery) -> Option<()> {
        let rates = self.rates_provider.get_rates().await?;
        self.repository
            .insert(query.source.to_string(), rates)
            .await?;
        Some(())
    }
}

impl FetchDataQueryHandler {
    pub fn new(
        repository: Arc<dyn IRatesRepository + Send + Sync>,
        rates_provider: Arc<dyn RatesProvider + Send + Sync>,
    ) -> Self {
        FetchDataQueryHandler {
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
pub trait IRatesRepository {
    async fn insert(&self, source: String, rates: Vec<CurrencyRate>) -> Option<()>;
}

#[async_trait::async_trait]
pub trait RatesProvider {
    async fn get_rates(&self) -> Option<Vec<CurrencyRate>>;
}
