use crate::config::Config;
use crate::error::AppError;
use anyhow::Result;
use mongodb::{Client, Database};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DatabaseConnection {
    client: Client,
    db: Database,
}

impl DatabaseConnection {
    pub async fn init(config: &Config) -> Result<Self, AppError> {
        let client = Client::with_uri_str(&config.mongodb_uri)
            .await
            .map_err(AppError::DatabaseError)?;

        // Test the connection
        client
            .database("admin")
            .run_command(mongodb::bson::doc! { "ping": 1 }, None)
            .await
            .map_err(AppError::DatabaseError)?;

        let db = client.database(&config.database_name);

        Ok(Self { client, db })
    }

    pub fn database(&self) -> Database {
        self.db.clone()
    }

    pub fn client(&self) -> Client {
        self.client.clone()
    }
}

pub type DbConn = Arc<DatabaseConnection>;

pub async fn connect_db(config: &Config) -> Result<DbConn, AppError> {
    let conn = DatabaseConnection::init(config).await?;
    Ok(Arc::new(conn))
}