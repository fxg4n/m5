use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize, Value};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde_json::Value as JsonValue;

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
    #[graphql(skip)]
    #[sqlx(skip)]
    pub audit_logs: Vec<AuditLog>,
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
            audit_logs: Vec::new(),
        }
    }
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
}

#[derive(SimpleObject, Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub resource: String,
    pub timestamp: DateTime<Utc>,
    pub details: Option<JsonValue>,
}

impl AuditLog {
    pub fn new(
        user_id: Uuid,
        action: String,
        resource: String,
        details: Option<JsonValue>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            action,
            resource,
            timestamp: Utc::now(),
            details,
        }
    }
}