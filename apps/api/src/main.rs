mod api;
mod config;
mod db;
mod error;
mod graphql;
mod models;

use std::net::SocketAddr;

use config::Config;
use db::pool::create_pool;
use error::AppError;
use graphql::schema::create_schema;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    
    let pool = create_pool(&config).await.map_err(|e| {
        tracing::error!("Failed to create database pool: {}", e);
        AppError::Database(e)
    })?;

    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to run migrations: {}", e);
            AppError::Database(e)
        })?;
    tracing::info!("Migrations completed successfully");

    let schema = create_schema(pool);

    let app = api::routes::create_router(schema)
        .layer(TraceLayer::new_for_http());

    let addr = config.server_addr();

    tracing::info!("Starting server at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}