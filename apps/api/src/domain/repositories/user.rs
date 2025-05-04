use sqlx::PgPool;
use crate::app::error::AppError;
use crate::domain::models::User;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(user)
    }

    pub async fn list(&self, limit: i32, offset: i32) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(users)
    }
}