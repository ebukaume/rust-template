use std::sync::Arc;

use axum::async_trait;

use crate::common::{Check, DatabaseDriver, RepositoryError};

pub struct HealthRepository {
    pub driver: Arc<DatabaseDriver>,
}

#[async_trait]
impl Check for HealthRepository {
    async fn check(&self) -> Result<(), RepositoryError> {
        self.driver.client.health().await?;

        Ok(())
    }
}

impl HealthRepository {
    pub fn new(driver: Arc<DatabaseDriver>) -> Self {
        Self { driver }
    }
}
