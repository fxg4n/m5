use sqlx::postgres::{PgPool, PgPoolOptions};
use anyhow::Result;
use std::time::Duration;
use tracing::{info, warn};

use crate::config::Config;

pub type DatabasePool = PgPool;

const MIN_CONNECTIONS: u32 = 2;
const MAX_CONNECTIONS: u32 = 30;
const MAX_LIFETIME: Duration = Duration::from_secs(30 * 60);
const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(30);
const IDLE_TIMEOUT: Duration = Duration::from_secs(10 * 60);

pub async fn create_pool(config: &Config) -> Result<DatabasePool> {
    info!("Initializing database connection pool...");

    let pool = PgPoolOptions::new()
        .min_connections(MIN_CONNECTIONS)
        .max_connections(MAX_CONNECTIONS)
        .max_lifetime(Some(MAX_LIFETIME))
        .acquire_timeout(ACQUIRE_TIMEOUT)
        .idle_timeout(Some(IDLE_TIMEOUT))
        .connect(&config.database.connection_string())
        .await?;

    check_database_connection(&pool).await?;

    info!(
        "Database connection pool initialized with {} max connections",
        MAX_CONNECTIONS
    );

    Ok(pool)
}

pub async fn check_database_connection(pool: &DatabasePool) -> Result<()> {
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => {
            info!("Database connection test successful");
            Ok(())
        }
        Err(e) => {
            warn!("Database connection test failed: {}", e);
            Err(anyhow::anyhow!("Database connection test failed: {}", e))
        }
    }
}

pub async fn close_pool(pool: DatabasePool) {
    info!("Closing database connection pool...");
    pool.close().await;
    info!("Database connection pool closed");
}