mod config;
mod db;
mod error;
mod graphql;
mod models;
mod routes;

use std::net::SocketAddr;

use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::db::connect_db;
use crate::graphql::create_schema;
use crate::routes::graphql_routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::init();
    tracing::info!("Configuration loaded: {:?}", config);

    let db_conn = connect_db(&config).await?;
    tracing::info!("Connected to MongoDB at {}", config.mongodb_uri);

    let schema = create_schema(db_conn);
    tracing::info!("GraphQL schema created");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/health", get(|| async { "OK" }))
        .merge(graphql_routes(schema))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}