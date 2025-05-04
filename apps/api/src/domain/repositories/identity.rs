use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::app::error::AppError;
use crate::domain::models::{User, UserSession, AuditLog};

pub struct IdentityRepository {
    pool: PgPool,
}

impl IdentityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}