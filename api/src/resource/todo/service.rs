use crate::{common::ApplicationError, resource::TodoRepository, util::validate_ulid};

use super::{CreateTodoRequest, Todo, TodoRepositoryImpl, UpdateTodoRequest};

type ServeiceResult<T> = Result<T, ApplicationError>;

pub struct TodoService<T>
where
    T: TodoRepository,
{
    repository: T,
}

impl TodoService<TodoRepositoryImpl> {
    pub fn new(repository: TodoRepositoryImpl) -> Self {
        Self { repository }
    }

    pub async fn get_todos(&self) -> ServeiceResult<Vec<Todo>> {
        let todos = self.repository.get_todos().await?;

        Ok(todos)
    }

    pub async fn get_todo_by_id(&self, id: String) -> ServeiceResult<Todo> {
        let id = validate_ulid(id)?;

        let todo = self.repository.get_todo_by_id(&id).await?;

        Ok(todo)
    }

    pub async fn create_todo(&self, todo: CreateTodoRequest) -> ServeiceResult<Todo> {
        let todo = Todo::from_request(todo);

        self.repository.create_todo(&todo).await?;

        Ok(todo)
    }

    pub async fn update_todo(&self, id: String, update: UpdateTodoRequest) -> ServeiceResult<Todo> {
        let id = validate_ulid(id)?;

        let todo = self.repository.get_todo_by_id(&id).await?;
        let updated_todo = update.merge(todo);

        let todo = self.repository.update_todo(&id, updated_todo).await?;

        Ok(todo)
    }

    pub async fn delete_todo(&self, id: String) -> ServeiceResult<Todo> {
        let id = validate_ulid(id)?;

        let todo = self.repository.delete_todo(&id).await?;

        Ok(todo)
    }

    pub async fn search_todo(&self, q: String) -> ServeiceResult<Vec<Todo>> {
        let todo = self.repository.search_todo(&q).await?;

        Ok(todo)
    }
}
