use std::sync::Arc;
use anyhow::Result;
use tokio::signal;

use crate::config::Config;
use crate::infrastructure::database;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db_pool: database::connection::DatabasePool,
}

pub async fn init() -> Result<Arc<AppState>> {
    let config = Config::load()?;

    crate::common::logging::init(
        &config.app.environment,
        "debug",
    );

    let db_pool = database::connection::create_pool(&config).await?;
    database::migrations::run_migrations(&db_pool).await?;

    let app_state = AppState {
        config,
        db_pool,
    };

    Ok(Arc::new(app_state))
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}