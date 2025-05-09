use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, crate::common::errors::AppError>;
pub type JsonMap = HashMap<String, JsonValue>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}