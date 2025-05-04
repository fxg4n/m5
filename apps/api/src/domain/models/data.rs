use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "source_type", rename_all = "lowercase")]
pub enum SourceType {
    Web,
    Api,
    Database,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "source_status", rename_all = "lowercase")]
pub enum SourceStatus {
    Active,
    Inactive,
}

#[derive(SimpleObject, Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Source {
    pub id: Uuid,
    pub name: String,
    #[sqlx(rename = "type")]
    pub source_type: SourceType,
    pub url: Option<String>,
    pub credentials: Option<JsonValue>,
    #[sqlx(rename = "status")]
    pub source_status: SourceStatus,
    pub last_accessed: Option<DateTime<Utc>>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub schedules: Vec<IngestionSchedule>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub jobs: Vec<IngestionJob>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub snapshots: Vec<Snapshot>,
}

impl Source {
    pub fn new(
        name: String,
        source_type: SourceType,
        url: Option<String>,
        credentials: Option<JsonValue>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            source_type,
            url,
            credentials,
            source_status: SourceStatus::Active,
            last_accessed: None,
            schedules: Vec::new(),
            jobs: Vec::new(),
            snapshots: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "job_status", rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(SimpleObject, Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct IngestionSchedule {
    pub id: Uuid,
    pub source_id: Uuid,
    pub cron_expression: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub source: Option<Source>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub jobs: Vec<IngestionJob>,
}

impl IngestionSchedule {
    pub fn new(
        source_id: Uuid,
        cron_expression: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_id,
            cron_expression,
            is_active: true,
            created_at: Utc::now(),
        }
    }

    pub fn with_relations(mut self, source: Source) -> Self {
        self.source = Some(source);
        self
    }
}

#[derive(SimpleObject, Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct IngestionJob {
    pub id: Uuid,
    pub source_id: Uuid,
    pub status: JobStatus,
    pub schedule_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub logs: Option<JsonValue>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub source: Option<Source>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub schedule: Option<IngestionSchedule>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub snapshots: Vec<Snapshot>,
}

impl IngestionJob {
    pub fn new(source_id: Uuid, schedule_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_id,
            status: JobStatus::Pending,
            schedule_id,
            started_at: Utc::now(),
            completed_at: None,
            logs: None,
        }
    }

    pub fn with_relations(mut self, source: Source, schedule: Option<IngestionSchedule>) -> Self {
        self.source = Some(source);
        self.schedule = schedule;
        self
    }
}

#[derive(SimpleObject, Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Snapshot {
    pub id: Uuid,
    pub source_id: Uuid,
    pub job_id: Uuid,
    pub content: JsonValue,
    pub ingested_at: DateTime<Utc>,
    pub metadata: Option<JsonValue>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub source: Option<Source>,
    #[graphql(skip)]
    #[sqlx(skip)]
    pub job: Option<IngestionJob>,
}

impl Snapshot {
    pub fn new(
        source_id: Uuid,
        job_id: Uuid,
        content: JsonValue,
        metadata: Option<JsonValue>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_id,
            job_id,
            content,
            ingested_at: Utc::now(),
            metadata,
            source: None,
            job: None,
        }
    }

    pub fn with_relations(mut self, source: Source, job: IngestionJob) -> Self {
        self.source = Some(source);
        self.job = Some(job);
        self
    }
}