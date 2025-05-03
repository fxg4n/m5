use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[graphql(skip)]
    pub password_hash: String,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
}

impl User {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, bio, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(users)
    }

    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, bio, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(user)
    }

    pub async fn find_by_username(username: &str, pool: &PgPool) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, bio, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(user)
    }

    pub async fn create(input: CreateUserInput, pool: &PgPool) -> Result<User> {
        // Hash the password
        let password_hash = hash_password(&input.password)?;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash, bio)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username, email, password_hash, bio, created_at, updated_at
            "#,
            input.username,
            input.email,
            password_hash,
            input.bio
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(user)
    }

    pub async fn update(id: Uuid, input: UpdateUserInput, pool: &PgPool) -> Result<User> {
        // First, get the existing user
        let user = Self::find_by_id(id, pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", id)))?;

        // Hash the password if provided
        let password_hash = if let Some(ref password) = input.password {
            Some(hash_password(password)?)
        } else {
            None
        };

        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET 
                username = COALESCE($1, username),
                email = COALESCE($2, email),
                password_hash = COALESCE($3, password_hash),
                bio = COALESCE($4, bio)
            WHERE id = $5
            RETURNING id, username, email, password_hash, bio, created_at, updated_at
            "#,
            input.username,
            input.email,
            password_hash,
            input.bio,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(user)
    }

    pub async fn delete(id: Uuid, pool: &PgPool) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn verify_password(&self, password: &str) -> Result<bool> {
        verify_password(password, &self.password_hash)
    }
}

fn hash_password(password: &str) -> Result<String> {
    let salt = argon2::password_hash::SaltString::generate(&mut rand::thread_rng());
    let argon2 = argon2::Argon2::default();
    
    let hash = argon2::PasswordHash::generate(
        argon2,
        password,
        &salt
    )
    .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?
    .to_string();
    
    Ok(hash)
}

fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let argon2 = argon2::Argon2::default();
    
    let hash = argon2::PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("Invalid password hash: {}", e)))?;
    
    Ok(argon2::PasswordVerifier::verify_password(&argon2, password.as_bytes(), &hash).is_ok())
}