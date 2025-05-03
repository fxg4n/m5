use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenv::dotenv().ok();
        
        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        config.try_deserialize()
    }

    pub fn server_addr(&self) -> SocketAddr {
        let ip = IpAddr::from_str(&self.server_host).expect("Invalid host address");
        SocketAddr::new(ip, self.server_port)
    }
}