use super::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub environment: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            host: env::get_var("APP_HOST")?,
            port: env::get_var("APP_PORT")?,
            environment: env::get_var_or("APP_ENV", "development".to_string()),
        })
    }
}
