use serde::{Deserialize, Serialize};

pub struct GetRatesQuery;

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
pub trait GetRatesQueryHandler: Send + Sync {
    async fn handle(&self, query: &GetRatesQuery) -> Option<Vec<CurrencyRate>>;
}
