use crate::{common::ApplicationError, resource::HealthRepository};

pub struct HealthService {
    repository: HealthRepository,
}

impl HealthService {
    pub fn new(repository: HealthRepository) -> Self {
        Self { repository }
    }

    pub async fn check_service_health(&self) -> Result<(), ApplicationError> {
        self.repository.check().await?;

        Ok(())
    }
}
