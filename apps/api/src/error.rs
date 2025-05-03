use async_graphql::Error as GraphQLError;
use mongodb::error::Error as MongoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] MongoError),

    #[error("GraphQL error: {0}")]
    GraphQLError(#[from] GraphQLError),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl From<AppError> for GraphQLError {
    fn from(error: AppError) -> Self {
        GraphQLError::new(error.to_string())
    }
}