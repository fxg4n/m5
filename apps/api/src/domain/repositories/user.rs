use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::app::error::AppError;
use crate::domain::models::{User, UserSession};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password, created_at, last_login
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

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password, created_at, last_login
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(user)
    }

    pub async fn update_last_login(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE users
            SET last_login = $1
            WHERE id = $2
            "#,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn create_session(
        &self,
        user_id: Uuid,
        token: String,
        expires_at: DateTime<Utc>,
        ip_address: String,
    ) -> Result<UserSession, AppError> {
        let session = sqlx::query_as!(
            UserSession,
            r#"
            INSERT INTO user_sessions (id, user_id, token, issued_at, expires_at, ip_address)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, token, issued_at, expires_at, ip_address
            "#,
            Uuid::new_v4(),
            user_id,
            token,
            Utc::now(),
            expires_at,
            ip_address
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(session)
    }

    pub async fn get_active_sessions(&self, user_id: Uuid) -> Result<Vec<UserSession>, AppError> {
        let sessions = sqlx::query_as!(
            UserSession,
            r#"
            SELECT id, user_id, token, issued_at, expires_at, ip_address
            FROM user_sessions
            WHERE user_id = $1 AND expires_at > $2
            "#,
            user_id,
            Utc::now()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(sessions)
    }

    pub async fn invalidate_session(&self, token: &str) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE user_sessions
            SET expires_at = $1
            WHERE token = $2
            "#,
            Utc::now(),
            token
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn find_session_by_token(&self, token: &str) -> Result<Option<UserSession>, AppError> {
        let session = sqlx::query_as!(
            UserSession,
            r#"
            SELECT id, user_id, token, issued_at, expires_at, ip_address
            FROM user_sessions
            WHERE token = $1 AND expires_at > $2
            "#,
            token,
            Utc::now()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(session)
    }
}