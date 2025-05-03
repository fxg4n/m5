use sqlx::PgPool;
use crate::app::error::AppError;
use super::models::{User, Post};

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
            "SELECT * FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(user)
    }

    pub async fn create(&self, username: String, email: String) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email)
            VALUES ($1, $2)
            RETURNING *
            "#,
            username,
            email
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(user)
    }
}

pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Post>, AppError> {
        let post = sqlx::query_as!(
            Post,
            "SELECT * FROM posts WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(post)
    }

    pub async fn create(
        &self,
        title: String,
        content: String,
        author_id: i32
    ) -> Result<Post, AppError> {
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (title, content, author_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            title,
            content,
            author_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(post)
    }
}