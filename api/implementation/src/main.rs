use std::net::SocketAddr;

use axum::{
    response::{IntoResponse, Redirect},
    Router, Server,
};
use resource::v1::health::controller::HealthController;

use tracing::info;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::{
    common::{Config, Constant, DatabaseDriver},
    resource::v1::{
        ApiDoc, HealthRepository, HealthService, TodoController, TodoRepositoryImpl, TodoService,
    },
    util::Tracing,
};

mod common;
mod resource;
mod util;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let _constant = Constant::new();
    let tracing = Tracing::init(&config);

    let database_driver = DatabaseDriver::init(&config)
        .await
        .expect("Unable to connect to DB!");

    let health_repository = HealthRepository::new(database_driver.clone());
    let todo_repository = TodoRepositoryImpl::new(database_driver.clone());

    let health_service = HealthService::new(health_repository);
    let todo_service = TodoService::new(todo_repository);

    let v1_prefix = "/v1";

    let health_controller = HealthController::new()
        .with_prefix(&format!("{}/health", &v1_prefix))
        .with_service(health_service)
        .build();

    let todo_controller = TodoController::new()
        .with_prefix(&format!("{}/todo", &v1_prefix))
        .with_service(todo_service)
        .build();

    let app = Router::new()
        .merge(health_controller)
        .merge(todo_controller)
        .merge(
            RapiDoc::with_openapi(&format!("{}/docs.json", &v1_prefix), ApiDoc::openapi())
                .path(&format!("{}/docs", &v1_prefix)),
        )
        .layer(tracing)
        .fallback(fallback);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    info!("Listening on {}", &addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Unable to start server on {}", &addr))
}

async fn fallback() -> impl IntoResponse {
    //
    Redirect::permanent("/v1/docs")
}
