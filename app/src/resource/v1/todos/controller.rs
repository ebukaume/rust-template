use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing, Json, Router,
};
use ulid::Ulid;

use crate::{
    common::{ApplicationError, ValidatedBody, ValidatedQuery},
    docs::v1::todos::{CreateTodoRequest, SearchTodoRequest, TodoResponse, UpdateTodoRequest},
    util::{Clock, IdGenerator},
};

use super::{TodoRepository, TodoService};

pub static TODO_TAG: &str = "Todo";

pub struct TodoController<R: TodoRepository, C: Clock, G: IdGenerator<Ulid>> {
    prefix: Option<String>,
    service: Option<TodoService<R, C, G>>,
}

impl<R: TodoRepository, C: Clock, G: IdGenerator<Ulid>> Default for TodoController<R, C, G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: TodoRepository, C: Clock, G: IdGenerator<Ulid>> TodoController<R, C, G> {
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

    pub fn with_service(mut self, service: TodoService<R, C, G>) -> Self {
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

#[utoipa::path(
    get,
    path = "/v1/todos",
    responses(
        (status = StatusCode::OK, description = "Get all Todos", body = [TodoResponse]),
        (status = StatusCode::NOT_FOUND, description = "Resource not found", body = Problem),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Problem),
    ),
    tag = TODO_TAG
)]
pub async fn get_todos<R, C, G>(
    State(service): State<Arc<TodoService<R, C, G>>>,
) -> Result<Json<Vec<TodoResponse>>, ApplicationError>
where
    R: TodoRepository,
    C: Clock,
    G: IdGenerator<Ulid>,
{
    let todos = service.get_todos().await?;

    let result: Vec<TodoResponse> = todos.into_iter().map(|t| t.into()).collect();

    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/v1/todos/{id}",
    params(("id", Path, example = "01HDS25AGAJ88WNXE5KZ3CN8KG")),
    responses(
        (status = StatusCode::OK, description = "Get Todo by Id", body = TodoResponse),
        (status = StatusCode::NOT_FOUND, description = "Resource not found", body = Problem),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Problem),
    ),
    tag = TODO_TAG
)]
pub async fn get_todo_by_id<R, C, G>(
    State(service): State<Arc<TodoService<R, C, G>>>,
    Path(todo_id): Path<String>,
) -> Result<TodoResponse, ApplicationError>
where
    R: TodoRepository,
    C: Clock,
    G: IdGenerator<Ulid>,
{
    let todo = service.get_todo_by_id(&todo_id).await?;

    Ok(todo.into())
}

#[utoipa::path(
    post,
    path = "/v1/todos",
    request_body = CreateTodoRequest,
    responses(
        (status = StatusCode::OK, description = "Create Todos", body = TodoResponse),
        (status = StatusCode::NOT_FOUND, description = "Resource not found", body = Problem),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Problem),
    ),
    tag = TODO_TAG
)]
pub async fn create_todo<R, C, G>(
    State(service): State<Arc<TodoService<R, C, G>>>,
    ValidatedBody(data): ValidatedBody<CreateTodoRequest>,
) -> Result<TodoResponse, ApplicationError>
where
    R: TodoRepository,
    C: Clock,
    G: IdGenerator<Ulid>,
{
    let todo = service.create_todo(data).await?;

    Ok(todo.into())
}

#[utoipa::path(
    delete,
    path = "/v1/todos/{id}",
    params(("id", Path, example = "01HDS25AGAJ88WNXE5KZ3CN8KG")),
    responses(
        (status = StatusCode::OK, description = "Delete Todo by Id", body = TodoResponse),
        (status = StatusCode::NOT_FOUND, description = "Resource not found", body = Problem),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Problem),
    ),
    tag = TODO_TAG
)]
pub async fn delete_todo<R, C, G>(
    State(service): State<Arc<TodoService<R, C, G>>>,
    Path(todo_id): Path<String>,
) -> Result<TodoResponse, ApplicationError>
where
    R: TodoRepository,
    C: Clock,
    G: IdGenerator<Ulid>,
{
    let todo = service.delete_todo(&todo_id).await?;

    Ok(todo.into())
}

#[utoipa::path(
    patch,
    path = "/v1/todos/{id}",
    params(("id", Path, example = "01HDS25AGAJ88WNXE5KZ3CN8KG")),
    request_body = UpdateTodoRequest,
    responses(
        (status = StatusCode::OK, description = "Update a Todo", body = TodoResponse),
        (status = StatusCode::NOT_FOUND, description = "Resource not found", body = Problem),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Problem),
    ),
    tag = TODO_TAG
)]
pub async fn update_todo<R, C, G>(
    State(service): State<Arc<TodoService<R, C, G>>>,
    Path(todo_id): Path<String>,
    ValidatedBody(update_data): ValidatedBody<UpdateTodoRequest>,
) -> Result<TodoResponse, ApplicationError>
where
    R: TodoRepository,
    C: Clock,
    G: IdGenerator<Ulid>,
{
    let todo = service.update_todo(&todo_id, update_data).await?;

    Ok(todo.into())
}

#[utoipa::path(
    get,
    path = "/v1/todos/search",
    params(SearchTodoRequest),
    responses(
        (status = StatusCode::OK, description = "Search for Todos based on subject adn description fields", body = [TodoResponse]),
        (status = StatusCode::NOT_FOUND, description = "Resource not found", body = Problem),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Problem),
    ),
    tag = TODO_TAG
)]
pub async fn search_todo<R, C, G>(
    State(service): State<Arc<TodoService<R, C, G>>>,
    ValidatedQuery(SearchTodoRequest { q }): ValidatedQuery<SearchTodoRequest>,
) -> Result<Json<Vec<TodoResponse>>, ApplicationError>
where
    R: TodoRepository,
    C: Clock,
    G: IdGenerator<Ulid>,
{
    let todos = service.search_todo(&q).await?;

    let result: Vec<TodoResponse> = todos.into_iter().map(|t| t.into()).collect();

    Ok(Json(result))
}
