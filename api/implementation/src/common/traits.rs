use axum::async_trait;

use super::RepositoryError;

#[async_trait]
pub trait Query<T> {
    async fn query(&self, q: String) -> Result<Option<T>, RepositoryError>;
}

#[async_trait]
pub trait Check {
    async fn check(&self) -> Result<(), RepositoryError>;
}
