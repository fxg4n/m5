use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::app::error::AppError;
use crate::domain::models::{Source, SourceType, SourceStatus};

pub struct DataRepository {
    pool: PgPool,
}

impl DataRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}