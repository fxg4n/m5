use async_graphql::{Context, Object, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &str {
        "Hello, world!"
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let pool = ctx.data::<PgPool>()?;
        let users = User::find_all(pool).await?;
        Ok(users)
    }

    async fn user_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<User>> {
        let pool = ctx.data::<PgPool>()?;
        let user = User::find_by_id(id, pool).await?;
        Ok(user)
    }

    async fn user_by_username(&self, ctx: &Context<'_>, username: String) -> Result<Option<User>> {
        let pool = ctx.data::<PgPool>()?;
        let user = User::find_by_username(&username, pool).await?;
        Ok(user)
    }
}