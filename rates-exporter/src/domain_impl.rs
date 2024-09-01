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
        let rates = self.rates_provider.get_rates(&query.coins_ids).await?;
        if rates.is_empty() {
            return Some(());
        }
        self.repository.insert(rates).await
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

#[derive(Deserialize, Serialize, Clone)]
pub struct CurrencyRate {
    pub _id: Key,
    pub image: String,
    pub last_updated: Option<String>,
    pub current_price: Option<f64>,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub price_change_percentage_24h: Option<f64>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Key {
    pub id: String,
    pub name: String,
    pub symbol: String,
}

#[async_trait::async_trait]
pub trait RatesRepository {
    async fn insert(&self, rates: Vec<CurrencyRate>) -> Option<()>;
}

#[async_trait::async_trait]
pub trait RatesProvider {
    async fn get_rates(&self, coins_ids: &str) -> Option<Vec<CurrencyRate>>;
}
