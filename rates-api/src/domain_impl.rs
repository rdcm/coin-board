use crate::domain::{CurrencyRate, GetRatesQuery, GetRatesQueryHandler};
use anyhow::Result;
use std::sync::Arc;

pub struct GetRatesQueryHandlerImpl {
    repository: Arc<dyn RatesRepository>,
}

impl GetRatesQueryHandlerImpl {
    pub fn new(repository: Arc<dyn RatesRepository + Send + Sync>) -> Self {
        GetRatesQueryHandlerImpl { repository }
    }
}

#[async_trait::async_trait]
impl GetRatesQueryHandler for GetRatesQueryHandlerImpl {
    async fn handle(&self, _query: &GetRatesQuery) -> Result<Vec<CurrencyRate>> {
        self.repository.get_rates().await
    }
}

#[async_trait::async_trait]
pub trait RatesRepository: Send + Sync + 'static {
    async fn get_rates(&self) -> Result<Vec<CurrencyRate>>;
}
