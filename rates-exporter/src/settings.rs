use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub provider: ProviderSettings,
}

#[derive(Debug, Deserialize)]
pub struct ProviderSettings {
    pub uri: String,
    pub fetch_delay_per_page_ms: u64,
    pub page_size: i32,
}

#[derive(Debug, Deserialize)]
pub struct DbSettings {
    pub db_name: String,
    pub uri: String,
}
