use async_graphql::Object;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health_check(&self) -> bool {
        true
    }
}