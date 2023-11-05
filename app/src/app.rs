use axum::{response::Redirect, Router};
use ulid::Ulid;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::{
    common::{Config, DatabaseDriver},
    resource::v1::{
        health::{HealthController, HealthRepository, HealthService},
        todos::{TodoController, TodoRepositoryImpl, TodoService},
        ApiDoc,
    },
    util::{Clock, IdGenerator, Telemetry},
};

pub struct AppBuilder<C: Clock, G: IdGenerator<Ulid>> {
    config: Option<Config>,
    database_driver: Option<DatabaseDriver>,
    clock: Option<C>,
    telemetry: Option<Telemetry>,
    id_generator: Option<G>,
}

impl<C: Clock + Clone, G: IdGenerator<Ulid> + Clone> Default for AppBuilder<C, G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Clock + Clone, G: IdGenerator<Ulid> + Clone> AppBuilder<C, G> {
    pub fn new() -> Self {
        AppBuilder {
            config: None,
            database_driver: None,
            clock: None,
            telemetry: None,
            id_generator: None,
        }
    }

    pub fn database_driver(mut self, database_driver: DatabaseDriver) -> Self {
        self.database_driver = Some(database_driver);

        self
    }

    pub fn clock(mut self, clock: C) -> Self {
        self.clock = Some(clock);

        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);

        self
    }

    pub fn telemetry(mut self, telemetry: Telemetry) -> Self {
        self.telemetry = Some(telemetry);

        self
    }

    pub fn id_generator(mut self, id_generator: G) -> Self {
        self.id_generator = Some(id_generator);

        self
    }

    pub async fn build(self) -> Result<Router, String> {
        let Some(database_driver) = self.database_driver else {
            return Err(String::from("Database driver not set"));
        };

        let Some(clock) = self.clock else {
            return Err(String::from("Clock not set"));
        };

        let Some(_config) = self.config else {
            return Err(String::from("Config not set"));
        };

        let Some(id_generator) = self.id_generator else {
            return Err(String::from("Id generator not set"));
        };

        let health_repository = HealthRepository::new(database_driver.clone());
        let todo_repository = TodoRepositoryImpl::new(database_driver.clone());

        let health_service = HealthService::new(health_repository);
        let todo_service = TodoService::new(todo_repository, clock.clone(), id_generator.clone());

        let v1_prefix = "/v1";

        let health_controller = HealthController::new()
            .with_prefix(&format!("{}/health", &v1_prefix))
            .with_service(health_service)
            .build();

        let todo_controller = TodoController::new()
            .with_prefix(&format!("{}/todos", &v1_prefix))
            .with_service(todo_service)
            .build();

        let app = Router::new()
            .merge(health_controller)
            .merge(todo_controller)
            .merge(
                RapiDoc::with_openapi(&format!("{}/docs.json", &v1_prefix), ApiDoc::openapi())
                    .path(&format!("{}/docs", &v1_prefix)),
            );

        if let Some(telemetry) = self.telemetry {
            let app = app
                .layer(telemetry)
                .fallback(Redirect::permanent("/v1/docs"));

            Ok(app)
        } else {
            let app = app.fallback(Redirect::permanent("/v1/docs"));

            Ok(app)
        }
    }
}
