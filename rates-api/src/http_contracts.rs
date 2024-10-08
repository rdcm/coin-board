use crate::domain::CurrencyRate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyRateModel {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub last_updated: Option<String>,
    pub current_price: Option<f64>,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub price_change_percentage_24h: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct RatesResponse {
    pub rates: Vec<CurrencyRateModel>,
}

impl CurrencyRate {
    fn into_model(self) -> CurrencyRateModel {
        CurrencyRateModel {
            id: self._id.id,
            symbol: self._id.symbol,
            name: self._id.name,
            image: self.image,
            last_updated: self.last_updated,
            current_price: self.current_price,
            high_24h: self.high_24h,
            low_24h: self.low_24h,
            price_change_24h: self.price_change_24h,
            price_change_percentage_24h: self.price_change_percentage_24h,
        }
    }
}

impl RatesResponse {
    pub fn from_currency_rates(rates: Vec<CurrencyRate>) -> Self {
        let rates_models = rates.into_iter().map(|m| m.into_model()).collect();

        RatesResponse {
            rates: rates_models,
        }
    }
}
