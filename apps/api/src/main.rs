mod app;
mod infrastructure;
mod api;
mod graphql;
mod domain;

use std::net::SocketAddr;

use app::config::Config;
use app::error::AppError;
use infrastructure::database::create_pool;
use graphql::schema::create_schema;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    init_logging();

    let config = Config::from_env()?;
    
    let pool = setup_database(&config).await?;

    let schema = create_schema(pool);

    run_server(config, schema).await?;

    Ok(())
}

fn init_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn setup_database(config: &Config) -> Result<sqlx::PgPool, AppError> {
    let pool = create_pool(config).await.map_err(|e| {
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

    Ok(pool)
}

async fn run_server(config: Config, schema: graphql::schema::Schema) -> Result<(), AppError> {
    let app = api::routes::create_router(schema)
        .layer(TraceLayer::new_for_http());

    let addr = config.server_addr();

    tracing::info!("Starting server at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| {
            tracing::error!("Server error: {}", e);
            AppError::Server(e.into())
        })?;

    Ok(())
}