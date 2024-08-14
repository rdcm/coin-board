use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub db_settings: DbSettings,
    pub api_settings: ApiSettings,
}

#[derive(Debug, Deserialize)]
pub struct DbSettings {
    pub db_name: String,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiSettings {
    pub host: String,
    pub port: i32,
}

impl ApiSettings {
    pub fn get_api_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
