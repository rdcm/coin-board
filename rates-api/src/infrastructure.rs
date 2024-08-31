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
        let limit = 200;
        let filter = doc! { "_id.id": { "$in": vec!["bitcoin", "ethereum", "solana", "dogecoin", "the-open-network", "binancecoin", "ripple", "pepe", "sun-token", "litecoin", "shiba-inu", "nyan-meme-coin"] }};
        let sort = doc! { "current_price": -1 };
        let cursor = self
            .collection
            .find(filter)
            .limit(limit)
            .sort(sort)
            .await
            .ok()?;

        match cursor.try_collect().await {
            Ok(v) => Some(v),
            Err(e) => {
                eprintln!("Find error: {:?}", e);
                None
            }
        }
    }
}
