use anyhow::Result;
use std::sync::Arc;

use crate::config::AppConfig;
use crate::infrastructure::database::connection::DatabasePool;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db_pool: DatabasePool,
}

pub async fn bootstrap(config: &AppConfig) -> Result<Arc<AppState>> {
    let db_pool = crate::infrastructure::database::connection::create_pool(config).await?;
    
    crate::infrastructure::database::migrations::run_migrations(&db_pool).await?;
    
    
    let app_state = AppState {
        config: config.clone(),
        db_pool,
    };
    
    Ok(Arc::new(app_state))
}