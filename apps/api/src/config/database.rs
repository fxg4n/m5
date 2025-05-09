use super::env;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub ssl_mode: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            host: env::get_var("DB_HOST")?,
            port: env::get_var("DB_PORT")?,
            username: env::get_var("DB_USER")?,
            password: env::get_var("DB_PASSWORD")?,
            database_name: env::get_var("DB_NAME")?,
            ssl_mode: env::get_var_or("DB_SSL_MODE", "prefer".to_string()),
        })
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode={}",
            self.username, self.password, self.host,
            self.port, self.database_name, self.ssl_mode
        )
    }
}