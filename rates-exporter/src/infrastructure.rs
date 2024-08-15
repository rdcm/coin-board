use crate::domain_impl::{CurrencyRate, RatesProvider, RatesRepository};
use mongodb::bson::doc;
use mongodb::{Client as MongoClient, Collection};
use reqwest::header::HeaderValue;

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

pub struct RatesProviderImpl {
    uri: String,
}

impl RatesProviderImpl {
    pub fn new(uri: String) -> Self {
        RatesProviderImpl { uri }
    }
}

#[async_trait::async_trait]
impl RatesProvider for RatesProviderImpl {
    async fn get_rates(&self, page: i32, page_size: i32) -> Option<Vec<CurrencyRate>> {
        let query = vec![
            ("page", page.to_string()),
            ("per_page", page_size.to_string()),
            ("vs_currency", "usd".to_string()),
        ];

        let client = reqwest::Client::new();
        let resp = client
            .get(&self.uri)
            .query(&query)
            .header("User-Agent", HeaderValue::from_static("Mozilla/5.0"))
            .send()
            .await
            .ok()?;

        if resp.status().is_success() {
            resp.json().await.ok()?
        } else {
            eprintln!(
                "code: {}, page: {}, resp: {}",
                resp.status(),
                page,
                resp.text().await.ok()?
            );
            None
        }
    }
}

#[async_trait::async_trait]
impl RatesRepository for RatesRepositoryImpl {
    async fn insert(&self, rates: Vec<CurrencyRate>) -> Option<()> {
        let ids = rates.iter().map(|r| r.id.clone()).collect::<Vec<String>>();
        let filter = doc! { "id": { "$in": ids } };

        _ = self.collection.delete_many(filter).await;
        match self.collection.insert_many(rates).await {
            Ok(_) => Some(()),
            Err(e) => {
                eprintln!("Bulk write error: {:?}", e);
                None
            }
        }
    }
}
