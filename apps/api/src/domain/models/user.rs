use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(SimpleObject, Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[graphql(skip)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: i32, username: String, password: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            username,
            password,
            created_at: now,
            updated_at: now,
        }
    }
}