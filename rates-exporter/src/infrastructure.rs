use crate::domain_impl::{CurrencyRate, RatesProvider, RatesRepository};
use mongodb::bson::doc;
use mongodb::{Client as MongoClient, Collection};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct RatesRepositoryImpl {
    collection: Collection<ProviderRates>,
}

#[derive(Deserialize, Serialize)]
pub struct ProviderRates {
    pub _id: String,
    pub rates: Vec<CurrencyRate>,
}

impl RatesRepositoryImpl {
    pub fn new(client: MongoClient, db_name: &str) -> Self {
        let collection: Collection<ProviderRates> = client.database(db_name).collection("rates");
        Self { collection }
    }
}

pub struct BinanceRatesProvider {
    uri: String,
}

impl BinanceRatesProvider {
    pub fn new(uri: String) -> Self {
        BinanceRatesProvider { uri }
    }
}

#[async_trait::async_trait]
impl RatesProvider for BinanceRatesProvider {
    async fn get_rates(&self) -> Option<Vec<CurrencyRate>> {
        let resp = reqwest::get(self.uri.to_string()).await;

        match resp {
            Ok(resp) => Some(resp.json().await.unwrap()),
            Err(_) => None,
        }
    }
}

#[async_trait::async_trait]
impl RatesRepository for RatesRepositoryImpl {
    async fn insert(&self, source: String, rates: Vec<CurrencyRate>) -> Option<()> {
        let filter = doc! { "_id": &source };
        let rates = ProviderRates { _id: source, rates };
        self.collection
            .replace_one(filter, rates)
            .upsert(true)
            .await
            .ok()?;
        Some(())
    }
}
