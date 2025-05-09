use anyhow::Result;
use sqlx::PgPool;
use tracing::info;

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;

    info!("Database migrations completed successfully");
    Ok(())
}