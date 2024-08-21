use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub endpoints: ApiSettings,
    pub cors: CorsSettings

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

#[derive(Debug, Deserialize)]
pub struct CorsSettings {
    pub methods: String,
    pub origin: String,
    pub headers: String,
    pub max_age: usize
}

impl ApiSettings {
    pub fn get_api_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
