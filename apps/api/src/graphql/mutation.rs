use async_graphql::{Context, InputObject, Object, Result};
use bson::{doc, oid::ObjectId, DateTime};
use futures::TryStreamExt;
use mongodb::Collection;

use crate::db::DbConn;
use crate::error::AppError;
use crate::models::{CreateUserInput, UpdateUserInput, User};

pub struct Mutation;

#[derive(InputObject)]
pub struct UserInput {
    pub name: String,
    pub email: String,
}

#[derive(InputObject)]
pub struct UserUpdateInput {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[Object]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>, input: UserInput) -> Result<User> {
        let db_conn = ctx.data::<DbConn>().unwrap();
        let user_collection: Collection<User> = db_conn.database().collection(User::collection_name());

        let user = User::new(input.name, input.email);

        user_collection
            .insert_one(&user, None)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(user)
    }

    async fn update_user(&self, ctx: &Context<'_>, input: UserUpdateInput) -> Result<Option<User>> {
        let db_conn = ctx.data::<DbConn>().unwrap();
        let user_collection: Collection<User> = db_conn.database().collection(User::collection_name());

        let object_id = ObjectId::parse_str(&input.id).map_err(|_| {
            AppError::ValidationError("Invalid ObjectId format".to_string())
        })?;

        let mut update_doc = doc! {};

        if let Some(name) = input.name {
            update_doc.insert("name", name);
        }

        if let Some(email) = input.email {
            update_doc.insert("email", email);
        }

        update_doc.insert("updated_at", DateTime::now());

        if update_doc.is_empty() {
            return Err(AppError::ValidationError("No fields to update".to_string()).into());
        }

        let update_doc = doc! { "$set": update_doc };

        let updated_user = user_collection
            .find_one_and_update(doc! { "_id": object_id }, update_doc, None)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(updated_user)
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let db_conn = ctx.data::<DbConn>().unwrap();
        let user_collection: Collection<User> = db_conn.database().collection(User::collection_name());

        let object_id = ObjectId::parse_str(&id).map_err(|_| {
            AppError::ValidationError("Invalid ObjectId format".to_string())
        })?;

        let result = user_collection
            .delete_one(doc! { "_id": object_id }, None)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(result.deleted_count > 0)
    }
}