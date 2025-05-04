use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::app::error::AppError;
use crate::domain::models::{Source, SourceType, SourceStatus};

pub struct AnalyticRepository {
    pool: PgPool,
}

impl AnalyticRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}