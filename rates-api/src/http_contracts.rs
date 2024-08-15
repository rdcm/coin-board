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
}

#[derive(Serialize, Deserialize)]
pub struct RatesResponse {
    pub rates: Vec<CurrencyRateModel>,
}

impl CurrencyRate {
    fn to_model(&self) -> CurrencyRateModel {
        CurrencyRateModel {
            id: self.id.clone(),
            symbol: self.symbol.clone(),
            name: self.name.clone(),
            image: self.image.clone(),
            last_updated: self.last_updated.clone(),
            current_price: self.current_price,
        }
    }
}

impl RatesResponse {
    pub fn from_currency_rates(rates: Vec<CurrencyRate>) -> Self {
        let rates_models = rates.iter().map(|m| m.to_model()).collect();

        RatesResponse {
            rates: rates_models,
        }
    }
}
