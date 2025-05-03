use axum::{
    middleware::Next,
    response::Response,
    http::Request,
};
use tower_http::trace::TraceLayer;

pub async fn auth<B>(req: Request<B>, next: Next<B>) -> Response {
    next.run(req).await
}

pub fn trace_layer() -> TraceLayer {
    TraceLayer::new_for_http()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use hyper::Body;

    #[tokio::test]
    async fn test_auth_middleware() {
        let req = Request::builder()
            .body(Body::empty())
            .unwrap();

        let next = Next::new(|request: Request<Body>| async move {
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap()
        });

        let response = auth(req, next).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}