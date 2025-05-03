use async_graphql::SimpleObject;
use bson::DateTime;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        let now = DateTime::now();
        Self {
            id: ObjectId::new(),
            name,
            email,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn collection_name() -> &'static str {
        "users"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserInput {
    pub name: Option<String>,
    pub email: Option<String>,
}