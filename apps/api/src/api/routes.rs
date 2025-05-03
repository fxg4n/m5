use axum::{
    Router,
    routing::get,
    extract::State,
    response::IntoResponse,
    http::StatusCode,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::graphql::schema::AppSchema;

pub fn create_router(schema: AppSchema) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .with_state(schema)
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

async fn graphql_playground() -> impl IntoResponse {
    async_graphql_axum::GraphQLPlaygroundResponse::new("/graphql")
}

async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}