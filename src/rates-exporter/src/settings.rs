use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub db_settings: DbSettings,
    pub provider_settings: ProviderSettings,
}

#[derive(Debug, Deserialize)]
pub struct ProviderSettings {
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct DbSettings {
    pub db_name: String,
    pub uri: String,
}
