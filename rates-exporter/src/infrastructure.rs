use crate::domain_impl::{CurrencyRate, Key, RatesProvider, RatesRepository};
use anyhow::{Context, Result};
use mongodb::bson::{doc, to_bson, Bson};
use mongodb::{Client as MongoClient, Collection};
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Clone)]
pub struct CoinGeckoCurrencyRate {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub image: String,
    pub last_updated: Option<String>,
    pub current_price: Option<f64>,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub price_change_percentage_24h: Option<f64>,
}

#[async_trait::async_trait]
impl RatesProvider for RatesProviderImpl {
    async fn get_rates(&self, coins_ids: &str) -> Result<Vec<CurrencyRate>> {
        let query = vec![("ids", coins_ids), ("vs_currency", "usd")];

        let client = reqwest::Client::new();
        let resp = client
            .get(&self.uri)
            .query(&query)
            .header(
                "User-Agent",
                HeaderValue::from_static("Mozilla/5.0"),
            )
            .send()
            .await
            .context("[rates-exporter] [coin-gecko-api] Fetch currency rates failed")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp
                .text()
                .await
                .context("[rates-exporter] [coin-gecko-api] Filed read bad response body")?;

            return Err(anyhow::anyhow!(
                "[rates-exporter] [coin-gecko-api] Received non-success response: code: {}, resp: {}",
                status,
                text
            ));
        }

        let exported_rates = resp
            .json::<Vec<CoinGeckoCurrencyRate>>()
            .await
            .context("[rates-exporter] [coin-gecko-api] Response deserialization failed")?;

        let rates: Vec<CurrencyRate> = exported_rates
            .into_iter()
            .map(move |cgcr| CurrencyRate {
                _id: Key {
                    id: cgcr.id,
                    name: cgcr.name,
                    symbol: cgcr.symbol,
                },
                image: cgcr.image,
                last_updated: cgcr.last_updated,
                current_price: cgcr.current_price,
                high_24h: cgcr.high_24h,
                low_24h: cgcr.low_24h,
                price_change_24h: cgcr.price_change_24h,
                price_change_percentage_24h: cgcr.price_change_percentage_24h,
            })
            .collect();

        Ok(rates)
    }
}

#[async_trait::async_trait]
impl RatesRepository for RatesRepositoryImpl {
    async fn insert(&self, rates: Vec<CurrencyRate>) -> Result<()> {
        let ids = rates
            .iter()
            .flat_map(|r| to_bson(&r._id))
            .collect::<Vec<Bson>>();

        let filter = doc! { "_id": { "$in": ids } };

        _ = self
            .collection
            .delete_many(filter)
            .await
            .context("[rates-exporter] [mongodb] Delete many failed")?;

        _ = self
            .collection
            .insert_many(rates)
            .await
            .context("[rates-exporter] [mongodb] Insert many failed")?;

        Ok(())
    }
}
