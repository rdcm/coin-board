use config::{Config, Environment, File};
use rates_exporter::service::Service;
use rates_exporter::settings::Settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Config::builder()
        .add_source(File::with_name("configs/rates-exporter-local.json"))
        .add_source(Environment::with_prefix("RATES_EXPORTER"))
        .build()?;

    let settings: Settings = settings.try_deserialize()?;

    Service::build(&settings)
        .await
        .run()
        .await
        .ok_or_else(|| "Rates export failed".into())
}
