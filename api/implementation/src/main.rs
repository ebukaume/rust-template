use std::{net::SocketAddr, sync::Arc};

use axum::{http::Uri, Router, Server};
use resource::health::controller::HealthController;
use tracing::info;

use crate::{
    common::{Config, Constant, DatabaseDriver},
    resource::{HealthRepository, HealthService, TodoController, TodoRepositoryImpl, TodoService},
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

    let database_client = Arc::new(
        DatabaseDriver::init(&config)
            .await
            .expect(&format!("Unable to connect to DB!")),
    );

    let health_repository = HealthRepository::new(Arc::clone(&database_client));
    let todo_repository = TodoRepositoryImpl::new(Arc::clone(&database_client));

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
        .layer(tracing)
        .fallback(fallback);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    info!("Listening on {}", &addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect(&format!("Unable to start server on {}", &addr))
}

async fn fallback(uri: Uri) -> String {
    format!("No route for {}\nPlease see the docs", uri)
}
