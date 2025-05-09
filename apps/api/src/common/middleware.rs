use axum::{
    middleware::Next,
    response::Response,
    http::Request,
};
use crate::common::logging::RequestId;

pub async fn request_id<B>(req: Request<B>, next: Next<B>) -> Response {
    let request_id = RequestId::new();
    tracing::Span::current().record("request_id", &request_id.as_str());
    
    next.run(req).await
}

pub async fn request_timer<B>(req: Request<B>, next: Next<B>) -> Response {
    let start = std::time::Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed();

    tracing::info!(
        duration_ms = duration.as_millis() as u64,
        "Request completed"
    );

    response
}