use std::net::SocketAddr;

use axum::{
    response::{IntoResponse, Redirect},
    Router, Server,
};
use resource::health::controller::HealthController;

use tracing::info;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::{
    common::{Config, Constant, DatabaseDriver},
    resource::{
        doc::ApiDoc, HealthRepository, HealthService, TodoController, TodoRepositoryImpl,
        TodoService,
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
    let tracing = Tracing::new(&config);

    let database_driver = DatabaseDriver::init(&config)
        .await
        .expect(&format!("Unable to connect to DB!"));

    let health_repository = HealthRepository::new(database_driver.clone());
    let todo_repository = TodoRepositoryImpl::new(database_driver.clone());

    let health_service = HealthService::new(health_repository);
    let todo_service = TodoService::new(todo_repository);

    let health_controller = HealthController::new()
        .with_prefix("/health")
        .with_service(health_service)
        .build();

    let todo_controller = TodoController::new()
        .with_prefix("/todo")
        .with_service(todo_service)
        .build();

    let app = Router::new()
        .merge(health_controller)
        .merge(todo_controller)
        .merge(RapiDoc::with_openapi("/docs.json", ApiDoc::openapi()).path("/docs"))
        .layer(tracing)
        .fallback(fallback);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    info!("Listening on {}", &addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect(&format!("Unable to start server on {}", &addr))
}

async fn fallback() -> impl IntoResponse {
    Redirect::permanent("/docs")
}
