use async_graphql::{Context, Object, Result};
use bson::{doc, oid::ObjectId};
use mongodb::Collection;

use crate::db::DbConn;
use crate::error::AppError;
use crate::models::User;

pub struct Query;

#[Object]
impl Query {
    async fn get_user(&self, ctx: &Context<'_>, id: String) -> Result<Option<User>> {
        let db_conn = ctx.data::<DbConn>().unwrap();
        let user_collection: Collection<User> = db_conn.database().collection(User::collection_name());

        let object_id = ObjectId::parse_str(&id).map_err(|_| {
            AppError::ValidationError("Invalid ObjectId format".to_string())
        })?;

        let user = user_collection
            .find_one(doc! { "_id": object_id }, None)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(user)
    }

    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let db_conn = ctx.data::<DbConn>().unwrap();
        let user_collection: Collection<User> = db_conn.database().collection(User::collection_name());

        let mut cursor = user_collection
            .find(None, None)
            .await
            .map_err(AppError::DatabaseError)?;

        let mut users = Vec::new();
        while let Some(user) = cursor
            .try_next()
            .await
            .map_err(AppError::DatabaseError)?
        {
            users.push(user);
        }

        Ok(users)
    }

    async fn health(&self) -> &'static str {
        "OK"
    }
}