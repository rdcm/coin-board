use serde::{Deserialize, Serialize};

pub struct GetRatesQuery;

#[derive(Deserialize, Serialize)]
pub struct CurrencyRate {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub last_updated: Option<String>,
    pub current_price: Option<f64>,
}

#[async_trait::async_trait]
pub trait GetRatesQueryHandler: Send + Sync {
    async fn handle(&self, query: &GetRatesQuery) -> Option<Vec<CurrencyRate>>;
}
