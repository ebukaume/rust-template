use crate::{common::ApplicationError, util::validate_ulid};

use api_documentation::v1::todo::{CreateTodoRequest, Todo, TodoModelUpdate, UpdateTodoRequest};

use super::TodoRepository;

type ServeiceResult<T> = Result<T, ApplicationError>;

pub struct TodoService<T>
where
    T: TodoRepository,
{
    repository: T,
}

impl<T> TodoService<T>
where
    T: TodoRepository,
{
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn get_todos(&self) -> ServeiceResult<Vec<Todo>> {
        let todos = self.repository.get_todos().await?;

        Ok(todos)
    }

    pub async fn get_todo_by_id(&self, id: &str) -> ServeiceResult<Todo> {
        let id = validate_ulid(id)?;

        let todo = self.repository.get_todo_by_id(&id).await?;

        Ok(todo)
    }

    pub async fn create_todo(&self, todo: CreateTodoRequest) -> ServeiceResult<Todo> {
        let todo = Todo::from_request(todo);

        let todo = self.repository.create_todo(todo).await?;

        Ok(todo)
    }

    pub async fn update_todo(&self, id: &str, update: UpdateTodoRequest) -> ServeiceResult<Todo> {
        let id = validate_ulid(id)?;
        let existing_todo = self.repository.get_todo_by_id(&id).await?;
        let updated_todo = TodoModelUpdate::merge(existing_todo, update);

        let todo = self.repository.update_todo(&id, updated_todo).await?;

        Ok(todo)
    }

    pub async fn delete_todo(&self, id: &str) -> ServeiceResult<Todo> {
        let id = validate_ulid(id)?;

        let todo = self.repository.delete_todo(&id).await?;

        Ok(todo)
    }

    pub async fn search_todo(&self, q: &str) -> ServeiceResult<Vec<Todo>> {
        let todo = self.repository.search_todo(q).await?;

        Ok(todo)
    }
}
