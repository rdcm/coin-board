use anyhow::{Context, Result};
use config::{Config, Environment};
use dotenvy::dotenv;
use rates_api::service::Service;
use rates_api::settings::Settings;
use tracing::instrument;
use tracing_subscriber::EnvFilter;

#[tokio::main]
#[instrument(ret, err)]
async fn main() -> Result<()> {
    _ = dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let settings = Config::builder()
        .add_source(Environment::with_prefix("RATES_API").separator("__"))
        .build()
        .context("[rates-api] [config] Failed to build config from env variables")?;

    let settings: Settings = settings
        .try_deserialize()
        .context("[rates-api] [config] Failed to deserialize config")?;

    Service::build(settings).await?.run().await?;

    Ok(())
}
