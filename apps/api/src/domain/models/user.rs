use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(SimpleObject, Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[graphql(skip)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub sessions: Vec<UserSession>,
}

#[derive(SimpleObject, Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub ip_address: String,
}

impl User {
    pub fn new(email: String, password: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            password,
            created_at: Utc::now(),
            last_login: None,
            sessions: Vec::new(),
        }
    }
}

impl UserSession {
    pub fn new(user_id: Uuid, token: String, expires_at: DateTime<Utc>, ip_address: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            token,
            issued_at: Utc::now(),
            expires_at,
            ip_address,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
