use async_graphql::{EmptySubscription, Schema, SchemaBuilder};
use sqlx::PgPool;

use super::queries::QueryRoot;
use super::mutations::MutationRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(pool: PgPool) -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(pool)
        .finish()
}