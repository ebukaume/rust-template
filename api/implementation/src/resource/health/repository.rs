use std::sync::Arc;

use crate::common::{DatabaseDriver, RepositoryError};

pub struct HealthRepository {
    pub driver: Arc<DatabaseDriver>,
}

impl HealthRepository {
    pub async fn check(&self) -> Result<(), RepositoryError> {
        self.driver.client.health().await?;

        Ok(())
    }
}

impl HealthRepository {
    pub fn new(driver: Arc<DatabaseDriver>) -> Self {
        Self { driver }
    }
}
