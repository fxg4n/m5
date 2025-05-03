use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::graphql::schema::AppSchema;

pub fn create_router(schema: AppSchema) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .with_state(schema)
}

async fn health_check() -> impl IntoResponse {
    "OK"
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}