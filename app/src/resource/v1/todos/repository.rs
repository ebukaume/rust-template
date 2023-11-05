use axum::async_trait;
use surrealdb::sql::Datetime;
use ulid::Ulid;

use crate::{
    common::{DatabaseDriver, RepositoryError},
    docs::v1::todos::{Todo, TodoModel, TodoModelUpdate},
};

type RepositoryResult<T> = Result<T, RepositoryError>;

#[async_trait]
pub trait TodoRepository: Send + Sync + 'static {
    async fn create_todo(&self, todo: Todo) -> RepositoryResult<TodoModel>;
    async fn get_todos(&self) -> RepositoryResult<Vec<TodoModel>>;
    async fn get_todo_by_id(&self, id: &Ulid) -> RepositoryResult<TodoModel>;
    async fn update_todo(
        &self,
        id: &Ulid,
        updated_todo: TodoModelUpdate,
    ) -> RepositoryResult<TodoModel>;
    async fn delete_todo(&self, id: &Ulid) -> RepositoryResult<TodoModel>;
    async fn search_todo(&self, q: &str) -> RepositoryResult<Vec<TodoModel>>;
}

pub struct TodoRepositoryImpl {
    pub driver: DatabaseDriver,
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn get_todos(&self) -> RepositoryResult<Vec<TodoModel>> {
        let mut response = self.driver.client.query("SELECT * FROM todo").await?;

        let result: Vec<TodoModel> = response.take(0)?;

        Ok(result)
    }

    async fn get_todo_by_id(&self, id: &Ulid) -> RepositoryResult<TodoModel> {
        let result: Option<TodoModel> =
            self.driver.client.select(("todo", &id.to_string())).await?;

        if let Some(todo) = result {
            return Ok(todo);
        }

        Err(RepositoryError::NotFound(id.to_string()))
    }

    async fn create_todo(&self, todo: Todo) -> RepositoryResult<TodoModel> {
        let query = r#"
            CREATE todo CONTENT {
                id: $id,
                subject: $subject,
                description: $description,
                due_date: $due_date,
                is_done: $is_done,
                created_at: $created_at,
                updated_at: $updated_at,
            }
        "#;
        let mut response = self
            .driver
            .client
            .query(query)
            .bind(("id", todo.id))
            .bind(("subject", todo.subject))
            .bind(("description", todo.description))
            .bind(("due_date", Datetime(todo.due_date)))
            .bind(("is_done", Some(todo.is_done)))
            .bind(("created_at", Datetime(todo.created_at)))
            .bind(("updated_at", Datetime(todo.updated_at)))
            .await?;

        let result: Option<TodoModel> = response.take(0)?;

        match result {
            Some(t) => Ok(t),
            None => Err(RepositoryError::InsertError(format!(
                "Todo({}) not returned after inserting into the DB",
                todo.id
            ))),
        }
    }

    async fn delete_todo(&self, id: &Ulid) -> RepositoryResult<TodoModel> {
        let result: Option<TodoModel> =
            self.driver.client.delete(("todo", &id.to_string())).await?;

        if let Some(todo) = result {
            return Ok(todo);
        }

        Err(RepositoryError::NotFound(id.to_string()))
    }

    async fn update_todo(
        &self,
        id: &Ulid,
        updated_todo: TodoModelUpdate,
    ) -> RepositoryResult<TodoModel> {
        let todo: Option<TodoModel> = self
            .driver
            .client
            .update(("todo", id.to_string()))
            .merge(updated_todo)
            .await?;

        if let Some(todo) = todo {
            return Ok(todo);
        }

        Err(RepositoryError::NotFound(id.to_string()))
    }

    async fn search_todo(&self, search_term: &str) -> RepositoryResult<Vec<TodoModel>> {
        let query = r#"
            SELECT *, search::score(1) * 2 + search::score(2) AS score FROM todo
            WHERE subject @1@ $search_term OR description @2@ $search_term
            ORDER BY score DESC
        "#;

        let mut response = self
            .driver
            .client
            .query(query)
            .bind(("search_term", search_term))
            .await?;

        let result: Vec<TodoModel> = response.take(0)?;

        Ok(result)
    }
}

impl TodoRepositoryImpl {
    pub fn new(driver: DatabaseDriver) -> Self {
        Self { driver }
    }
}
