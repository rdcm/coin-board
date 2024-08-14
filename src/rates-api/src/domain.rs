use serde::{Deserialize, Serialize};

pub struct GetRatesQuery<'a> {
    pub source: &'a str,
}

#[derive(Deserialize, Serialize)]
pub struct CurrencyRate {
    pub symbol: String,
    pub price: String,
}

#[async_trait::async_trait]
pub trait GetRatesQueryHandler: Send + Sync {
    async fn handle(&self, query: &GetRatesQuery) -> Option<Vec<CurrencyRate>>;
}
