use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub provider: ProviderSettings,
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
