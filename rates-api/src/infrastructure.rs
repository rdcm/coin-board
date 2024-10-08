use crate::domain::CurrencyRate;
use crate::domain_impl::RatesRepository;
use anyhow::{Context, Result};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{Client as MongoClient, Collection};

#[derive(Clone)]
pub struct RatesRepositoryImpl {
    collection: Collection<CurrencyRate>,
}

impl RatesRepositoryImpl {
    pub fn new(client: MongoClient, db_name: &str) -> Self {
        let collection: Collection<CurrencyRate> = client.database(db_name).collection("rates");
        Self { collection }
    }
}

#[async_trait::async_trait]
impl RatesRepository for RatesRepositoryImpl {
    async fn get_rates(&self) -> Result<Vec<CurrencyRate>> {
        let filter = doc! {};
        let sort = doc! { "current_price": -1 };

        let cursor = self
            .collection
            .find(filter)
            .sort(sort)
            .await
            .context("[rates-api] [mongodb] Failed to find documents in the collection")?;

        let rates = cursor
            .try_collect()
            .await
            .context("[rates-api] [mongodb] Failed to collect rates from the cursor")?;

        Ok(rates)
    }
}
