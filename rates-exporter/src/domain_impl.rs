use crate::domain::{FetchDataQuery, FetchRatesQueryHandler};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct FetchRatesQueryHandlerImpl {
    pub repository: Arc<dyn RatesRepository + Send + Sync>,
    pub rates_provider: Arc<dyn RatesProvider + Send + Sync>,
}

#[async_trait::async_trait]
impl FetchRatesQueryHandler for FetchRatesQueryHandlerImpl {
    async fn handle(&self, query: &FetchDataQuery) -> Option<()> {
        for page in 1..=100 {
            let rates = self.rates_provider.get_rates(page, query.page_size).await?;
            if rates.is_empty() {
                return Some(());
            }
            self.repository.insert(rates).await?;

            sleep(Duration::from_millis(query.fetch_delay_ms)).await;
        }

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

#[derive(Deserialize, Serialize, Clone)]
pub struct CurrencyRate {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub last_updated: Option<String>,
    pub current_price: Option<f64>,
}

#[async_trait::async_trait]
pub trait RatesRepository {
    async fn insert(&self, rates: Vec<CurrencyRate>) -> Option<()>;
}

#[async_trait::async_trait]
pub trait RatesProvider {
    async fn get_rates(&self, page: i32, page_size: i32) -> Option<Vec<CurrencyRate>>;
}
