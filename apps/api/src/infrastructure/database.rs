use sqlx::postgres::{PgPool, PgPoolOptions};
use crate::app::config::Config;

pub async fn create_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
}

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn check_connection(&self) -> Result<(), sqlx::Error> {
        self.pool.acquire().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::config::Config;

    #[tokio::test]
    async fn test_database_connection() {
        let config = Config {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set for testing"),
            host: "localhost".to_string(),
            port: 3000,
        };

        let pool = create_pool(&config).await.expect("Failed to create pool");
        let db = Database::new(pool);
        
        db.check_connection()
            .await
            .expect("Failed to connect to database");
    }
}