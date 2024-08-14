use crate::domain::CurrencyRate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyRateModel {
    pub symbol: String,
    pub price: String,
}

#[derive(Serialize, Deserialize)]
pub struct RatesResponse {
    pub rates: HashMap<String, Vec<CurrencyRateModel>>,
}

impl CurrencyRate {
    fn to_model(&self) -> CurrencyRateModel {
        CurrencyRateModel {
            symbol: self.symbol.clone(),
            price: self.price.clone(),
        }
    }
}

impl RatesResponse {
    pub fn from_currency_rates(source: String, rates: Vec<CurrencyRate>) -> Self {
        let mut map = HashMap::new();
        map.insert(source, rates.iter().map(|m| m.to_model()).collect());

        RatesResponse { rates: map }
    }
}
