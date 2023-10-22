use std::sync::Arc;

use axum::async_trait;
use ulid::Ulid;

use crate::{
    common::{DatabaseDriver, RepositoryError},
    resource::TodoModel,
};

use super::{Todo, UpdateTodoRequest};

type RepositoryResult<T> = Result<T, RepositoryError>;

#[async_trait]
pub trait TodoRepository {
    async fn create_todo(&self, todo: &Todo) -> Result<(), RepositoryError>;
    async fn get_todos(&self) -> RepositoryResult<Vec<Todo>>;
    async fn get_todo_by_id(&self, id: &Ulid) -> RepositoryResult<Todo>;
    async fn update_todo(
        &self,
        id: &Ulid,
        updated_todo: UpdateTodoRequest,
    ) -> RepositoryResult<Todo>;
    async fn delete_todo(&self, id: &Ulid) -> RepositoryResult<Todo>;
    async fn search_todo(&self, q: &str) -> RepositoryResult<Vec<Todo>>;
}

pub struct TodoRepositoryImpl {
    pub driver: Arc<DatabaseDriver>,
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn get_todos(&self) -> RepositoryResult<Vec<Todo>> {
        let mut response = self.driver.client.query("SELECT * FROM todo").await?;

        let result: Vec<TodoModel> = response.take(0)?;

        Ok(result.into_iter().map(|t| t.into()).collect())
    }

    async fn get_todo_by_id(&self, id: &Ulid) -> RepositoryResult<Todo> {
        let result: Option<TodoModel> =
            self.driver.client.select(("todo", &id.to_string())).await?;

        if let Some(todo) = result {
            return Ok(todo.into());
        }

        Err(RepositoryError::NotFound(id.to_string()))
    }

    async fn create_todo(&self, todo: &Todo) -> RepositoryResult<()> {
        self.driver
            .client
            .query("CREATE todo CONTENT $todo")
            .bind(("todo", todo))
            .await?;

        Ok(())
    }

    async fn delete_todo(&self, id: &Ulid) -> RepositoryResult<Todo> {
        let result: Option<TodoModel> =
            self.driver.client.delete(("todo", &id.to_string())).await?;

        if let Some(todo) = result {
            return Ok(todo.into());
        }

        Err(RepositoryError::NotFound(id.to_string()))
    }

    async fn update_todo(
        &self,
        id: &Ulid,
        updated_todo: UpdateTodoRequest,
    ) -> RepositoryResult<Todo> {
        let todo: Option<TodoModel> = self
            .driver
            .client
            .update(("todo", &id.to_string()))
            .merge(updated_todo)
            .await?;

        if let Some(todo) = todo {
            return Ok(todo.into());
        }

        Err(RepositoryError::NotFound(id.to_string()))
    }

    async fn search_todo(&self, q: &str) -> RepositoryResult<Vec<Todo>> {
        let mut response = self
            .driver
            .client
            .query("SELECT search::score(1) AS score FROM todo WHERE subject @1@ to ORDER BY score DESC;")
            .bind(("query", q))
            .await?;

        let result: Vec<TodoModel> = response.take(0)?;

        Ok(result.into_iter().map(|t| t.into()).collect())
    }
}

impl TodoRepositoryImpl {
    pub fn new(driver: Arc<DatabaseDriver>) -> Self {
        Self { driver }
    }
}
