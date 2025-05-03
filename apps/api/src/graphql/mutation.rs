use async_graphql::{Context, InputObject, Object, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::{CreateUserInput as DbCreateUserInput, UpdateUserInput as DbUpdateUserInput, User};

pub struct Mutation;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
}

#[Object]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let pool = ctx.data::<PgPool>()?;
        
        let db_input = DbCreateUserInput {
            username: input.username,
            email: input.email,
            password: input.password,
            bio: input.bio,
        };
        
        let user = User::create(db_input, pool).await?;
        Ok(user)
    }

    async fn update_user(&self, ctx: &Context<'_>, id: Uuid, input: UpdateUserInput) -> Result<User> {
        let pool = ctx.data::<PgPool>()?;
        
        let db_input = DbUpdateUserInput {
            username: input.username,
            email: input.email,
            password: input.password,
            bio: input.bio,
        };
        
        let user = User::update(id, db_input, pool).await?;
        Ok(user)
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        let result = User::delete(id, pool).await?;
        Ok(result)
    }
}