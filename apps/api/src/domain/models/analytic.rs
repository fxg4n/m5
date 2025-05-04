use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde_json::Value as JsonValue;

use super::data::Source;

#[derive(SimpleObject, Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Dataset {
    pub id: Uuid,
    pub name: String,
    pub size: i32,
    pub created_at: DateTime<Utc>,
    pub metadata: Option<JsonValue>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub sources: Vec<Source>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub models: Vec<ModelRegistry>,
}

impl Dataset {
    pub fn new(name: String, size: i32, metadata: Option<JsonValue>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            size,
            created_at: Utc::now(),
            metadata,
            sources: Vec::new(),
            models: Vec::new(),
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct DatasetSource {
    pub id: Uuid,
    pub dataset_id: Uuid,
    pub source_id: Uuid,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub dataset: Option<Dataset>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub source: Option<Source>,
}

impl DatasetSource {
    pub fn new(dataset_id: Uuid, source_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            dataset_id,
            source_id,
            dataset: None,
            source: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "model_type", rename_all = "lowercase")]
pub enum ModelType {
    Classification,
    Regression,
    Nlp,
}

#[derive(SimpleObject, Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct ModelRegistry {
    pub id: Uuid,
    pub name: String,
    #[sqlx(rename = "type")]
    pub model_type: ModelType,
    pub version: String,
    pub dataset_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub deployed_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub dataset: Option<Dataset>,
}

impl ModelRegistry {
    pub fn new(
        name: String,
        model_type: ModelType,
        version: String,
        dataset_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            model_type,
            version,
            dataset_id,
            created_at: Utc::now(),
            deployed_at: None,
            dataset: None,
        }
    }
}