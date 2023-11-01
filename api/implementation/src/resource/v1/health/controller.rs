use std::sync::Arc;

use api_documentation::v1::health::HealthStatusResponse;
use axum::{extract::State, routing, Router};

use crate::common::ApplicationError;

use super::service::HealthService;

pub struct HealthController {
    prefix: Option<String>,
    service: Option<HealthService>,
}

impl HealthController {
    pub fn new() -> Self {
        Self {
            prefix: None,
            service: None,
        }
    }

    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());

        self
    }

    pub fn with_service(mut self, service: HealthService) -> Self {
        self.service = Some(service);

        self
    }

    pub fn build(self) -> Router {
        let prefix = self.prefix.expect("prefix not set");
        let service = Arc::new(self.service.expect("service not set"));

        let router = Router::new()
            .route("/", routing::get(health_status))
            .with_state(service);

        Router::new().nest(&prefix, router)
    }
}

async fn health_status(
    State(service): State<Arc<HealthService>>,
) -> Result<HealthStatusResponse, ApplicationError> {
    let database_status = service
        .check_service_health()
        .await
        .map_or(String::from("NOK"), |_| String::from("OK"));

    Ok(HealthStatusResponse {
        api: String::from("OK"),
        database: database_status,
    })
}
