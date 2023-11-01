use crate::common::{DatabaseDriver, RepositoryError};

#[derive(Clone)]
pub struct HealthRepository {
    pub driver: DatabaseDriver,
}

impl HealthRepository {
    pub async fn check(&self) -> Result<(), RepositoryError> {
        self.driver.client.health().await?;

        Ok(())
    }
}

impl HealthRepository {
    pub fn new(driver: DatabaseDriver) -> Self {
        Self { driver }
    }
}
