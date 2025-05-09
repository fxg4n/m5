pub mod app;
pub mod database;
pub mod services;
pub mod env;

use app::AppConfig;
use database::DatabaseConfig;
use services::ServicesConfig;

#[derive(Debug, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub services: ServicesConfig,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            app: AppConfig::from_env()?,
            database: DatabaseConfig::from_env()?,
            services: ServicesConfig::from_env()?,
        })
    }
}
