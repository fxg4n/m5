use async_graphql::{EmptySubscription, Schema};

use crate::db::DbConn;
use crate::graphql::{Mutation, Query};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(db_conn: DbConn) -> AppSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(db_conn)
        .finish()
}