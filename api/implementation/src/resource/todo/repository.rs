use api_documentation::todo::{Todo, TodoModel, TodoModelUpdate};
use axum::async_trait;
use chrono::Utc;
use surrealdb::sql::Datetime;
use ulid::Ulid;

use crate::common::{DatabaseDriver, RepositoryError};

type RepositoryResult<T> = Result<T, RepositoryError>;

#[async_trait]
pub trait TodoRepository {
    async fn create_todo(&self, todo: Todo) -> RepositoryResult<Todo>;
    async fn get_todos(&self) -> RepositoryResult<Vec<Todo>>;
    async fn get_todo_by_id(&self, id: &Ulid) -> RepositoryResult<Todo>;
    async fn update_todo(&self, id: &Ulid, updated_todo: TodoModelUpdate)
        -> RepositoryResult<Todo>;
    async fn delete_todo(&self, id: &Ulid) -> RepositoryResult<Todo>;
    async fn search_todo(&self, q: &str) -> RepositoryResult<Vec<Todo>>;
}

pub struct TodoRepositoryImpl {
    pub driver: DatabaseDriver,
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

    async fn create_todo(&self, todo: Todo) -> RepositoryResult<Todo> {
        println!("{}", &todo.id);
        let creation_date = Utc::now();

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
            .bind(("created_at", Datetime(creation_date)))
            .bind(("updated_at", Datetime(creation_date)))
            .await?;

        let todo: Option<TodoModel> = response.take(0)?;

        Ok(todo.unwrap().into())
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
        updated_todo: TodoModelUpdate,
    ) -> RepositoryResult<Todo> {
        let todo: Option<TodoModel> = self
            .driver
            .client
            .update(("todo", id.to_string()))
            .merge(updated_todo)
            .await?;

        if let Some(todo) = todo {
            return Ok(todo.into());
        }

        Err(RepositoryError::NotFound(id.to_string()))
    }

    async fn search_todo(&self, search_term: &str) -> RepositoryResult<Vec<Todo>> {
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

        Ok(result.into_iter().map(|t| t.into()).collect())
    }
}

impl TodoRepositoryImpl {
    pub fn new(driver: DatabaseDriver) -> Self {
        Self { driver }
    }
}
