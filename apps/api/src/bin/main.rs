use std::net::SocketAddr;
use tracing::info;

use m5::{init, shutdown_signal};
use m5::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let app_state = init().await?;
    
    let addr = SocketAddr::from(([0, 0, 0, 0], app_state.config.app.port));
    
    info!(
        "Starting server on {}:{} in {} mode",
        app_state.config.app.host,
        app_state.config.app.port,
        app_state.config.app.environment
    );

    let app = app_state.config.app.port;

    let server = axum::Server::bind(&addr)
        .serve(app)
        .with_graceful_shutdown(shutdown_signal());

    info!("Server started successfully");
    
    if let Err(e) = server.await {
        tracing::error!("Server error: {}", e);
    }

    info!("Shutting down...");
    api::infrastructure::database::connection::close_pool(app_state.db_pool).await;
    info!("Shutdown complete");

    Ok(())
}