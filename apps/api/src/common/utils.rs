use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::time::{Duration, SystemTime};
use tokio::time::{sleep, Duration};

pub fn timestamp_now() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

pub fn datetime_to_timestamp(datetime: DateTime<Utc>) -> i64 {
    datetime.timestamp()
}

pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 3])
    }
}

pub async fn with_timeout<F, T>(future: F, timeout: Duration) -> Result<T, &'static str>
where
    F: Future<Output = T>,
{
    tokio::time::timeout(timeout, future)
        .await
        .map_err(|_| "operation timed out")?
}

pub fn mask_sensitive_data(data: &str) -> String {
    if data.is_empty() {
        return String::new();
    }
    
    let visible_chars = 4;
    let total_len = data.len();
    
    if total_len <= visible_chars * 2 {
        return "*".repeat(total_len);
    }
    
    format!(
        "{}{}{}",
        &data[..visible_chars],
        "*".repeat(total_len - (visible_chars * 2)),
        &data[total_len - visible_chars..]
    )
}

pub fn chunk_vec<T: Clone>(vec: &[T], chunk_size: usize) -> Vec<Vec<T>> {
    vec.chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

pub async fn retry<F, Fut, T, E>(mut f: F, retries: u32, delay: Duration) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    for _ in 0..retries {
        match f().await {
            Ok(val) => return Ok(val),
            Err(_) => sleep(delay).await,
        }
    }
    f().await
}
