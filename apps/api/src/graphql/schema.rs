use async_graphql::{EmptySubscription, Schema};
use sqlx::PgPool;

use super::mutation::Mutation;
use super::query::Query;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(pool: PgPool) -> AppSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .finish()
}