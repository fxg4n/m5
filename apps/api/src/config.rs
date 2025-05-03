use std::env;
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub mongodb_uri: String,
    pub database_name: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Self {
        dotenv().ok();

        let mongodb_uri = env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        
        let database_name = env::var("DATABASE_NAME")
            .unwrap_or_else(|_| "rust_axum_graphql_db".to_string());
        
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .unwrap_or(8000);

        Self {
            mongodb_uri,
            database_name,
            port,
        }
    }
}