use crate::domain::CurrencyRate;
use crate::domain_impl::RatesRepository;
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

#[async_trait::async_trait]
impl RatesRepository for RatesRepositoryImpl {
    async fn get_rates(&self, source: &str) -> Option<Vec<CurrencyRate>> {
        let filter = doc! { "_id": &source };
        let res = self.collection.find_one(filter).await.ok()??;

        Some(res.rates)
    }
}
