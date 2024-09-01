use crate::domain::CurrencyRate;
use crate::domain_impl::RatesRepository;
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
    async fn get_rates(&self) -> Option<Vec<CurrencyRate>> {
        let sort = doc! { "current_price": -1 };
        let cursor = self.collection.find(doc! {}).sort(sort).await.ok()?;

        match cursor.try_collect().await {
            Ok(v) => Some(v),
            Err(e) => {
                eprintln!("Find error: {:?}", e);
                None
            }
        }
    }
}
