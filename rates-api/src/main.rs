use anyhow::{Context, Result};
use config::{Config, Environment};
use dotenvy::dotenv;
use rates_api::service::Service;
use rates_api::settings::Settings;

#[tokio::main]
async fn main() -> Result<()> {
    _ = dotenv();

    std::env::set_var("RUST_LOG", "error");
    env_logger::init();

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
