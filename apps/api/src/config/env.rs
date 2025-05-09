use std::env;
use dotenv::dotenv;

pub fn init() {
    dotenv().ok();
}

pub fn get_var<T: std::str::FromStr>(key: &str) -> Result<T, Box<dyn std::error::Error>> {
    env::var(key)?.parse::<T>().map_err(|_| format!("Failed to parse {}", key).into())
}

pub fn get_var_or<T: std::str::FromStr>(key: &str, default: T) -> T {
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

pub fn get_env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}