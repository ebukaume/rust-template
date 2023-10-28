use std::sync::Arc;

use api_documentation::todo::{
    CreateTodoRequest, SearchTodoRequest, TodoResponse, UpdateTodoRequest,
};
use axum::{
    extract::{Path, State},
    routing, Json, Router,
};

use crate::{
    common::{ApplicationError, ValidatedBody, ValidatedQuery},
    resource::TodoService,
};

use super::TodoRepositoryImpl;

pub struct TodoController {
    prefix: Option<String>,
    service: Option<TodoService<TodoRepositoryImpl>>,
}

impl TodoController {
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

    pub fn with_service(mut self, service: TodoService<TodoRepositoryImpl>) -> Self {
        self.service = Some(service);

        self
    }

    pub fn build(self) -> Router {
        let prefix = self.prefix.expect("prefix not set");
        let service = Arc::new(self.service.expect("service not set"));

        let router = Router::new()
            .route("/", routing::get(get_todos))
            .route("/", routing::post(create_todo))
            .route("/:id", routing::get(get_todo_by_id))
            .route("/:id", routing::patch(update_todo))
            .route("/:id", routing::delete(delete_todo))
            .route("/search", routing::get(search_todo))
            .with_state(service);

        Router::new().nest(&prefix, router)
    }
}

async fn get_todos(
    State(service): State<Arc<TodoService<TodoRepositoryImpl>>>,
) -> Result<Json<Vec<TodoResponse>>, ApplicationError> {
    let todos = service.get_todos().await?;

    let result: Vec<TodoResponse> = todos.into_iter().map(|t| t.into()).collect();

    Ok(Json(result))
}

async fn get_todo_by_id(
    State(service): State<Arc<TodoService<TodoRepositoryImpl>>>,
    Path(todo_id): Path<String>,
) -> Result<TodoResponse, ApplicationError> {
    let todo = service.get_todo_by_id(&todo_id).await?;

    Ok(todo.into())
}

async fn create_todo(
    State(service): State<Arc<TodoService<TodoRepositoryImpl>>>,
    ValidatedBody(data): ValidatedBody<CreateTodoRequest>,
) -> Result<TodoResponse, ApplicationError> {
    let todo = service.create_todo(data).await?;

    Ok(todo.into())
}

async fn delete_todo(
    State(service): State<Arc<TodoService<TodoRepositoryImpl>>>,
    Path(todo_id): Path<String>,
) -> Result<TodoResponse, ApplicationError> {
    let todo = service.delete_todo(&todo_id).await?;

    Ok(todo.into())
}

async fn update_todo(
    State(service): State<Arc<TodoService<TodoRepositoryImpl>>>,
    Path(todo_id): Path<String>,
    ValidatedBody(update_data): ValidatedBody<UpdateTodoRequest>,
) -> Result<TodoResponse, ApplicationError> {
    let todo = service.update_todo(&todo_id, update_data).await?;

    Ok(todo.into())
}

async fn search_todo(
    State(service): State<Arc<TodoService<TodoRepositoryImpl>>>,
    ValidatedQuery(SearchTodoRequest { q }): ValidatedQuery<SearchTodoRequest>,
) -> Result<Json<Vec<TodoResponse>>, ApplicationError> {
    let todos = service.search_todo(&q).await?;

    let result: Vec<TodoResponse> = todos.into_iter().map(|t| t.into()).collect();

    Ok(Json(result))
}
