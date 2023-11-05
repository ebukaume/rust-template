use crate::{
    common::ApplicationError,
    docs::v1::todos::{CreateTodoRequest, Todo, TodoModel, TodoModelUpdate, UpdateTodoRequest},
    util::{Clock, IdGenerator},
};

use ulid::Ulid;

use super::TodoRepository;

type ServeiceResult<T> = Result<T, ApplicationError>;

pub struct TodoService<R, C, G>
where
    R: TodoRepository,
    C: Clock,
    G: IdGenerator<Ulid>,
{
    repository: R,
    clock: C,
    id_generator: G,
}

impl<T, C, G> TodoService<T, C, G>
where
    T: TodoRepository,
    C: Clock,
    G: IdGenerator<Ulid>,
{
    pub fn new(repository: T, clock: C, id_generator: G) -> Self {
        Self {
            repository,
            clock,
            id_generator,
        }
    }

    pub async fn get_todos(&self) -> ServeiceResult<Vec<Todo>> {
        let todos_model = self.repository.get_todos().await?;

        let reasult: Result<Vec<Todo>, ApplicationError> = todos_model
            .into_iter()
            .map(|t| self.model_to_domain(t))
            .collect();

        match reasult {
            Ok(todos) => Ok(todos),
            Err(msg) => Err(msg),
        }
    }

    pub async fn get_todo_by_id(&self, id: &str) -> ServeiceResult<Todo> {
        let id = self.id_generator.parse(id)?;

        let todo = self.repository.get_todo_by_id(&id).await?;

        self.model_to_domain(todo)
    }

    pub async fn create_todo(&self, todo: CreateTodoRequest) -> ServeiceResult<Todo> {
        let todo = self.request_to_domain(todo);

        let todo = self.repository.create_todo(todo).await?;

        self.model_to_domain(todo)
    }

    pub async fn update_todo(&self, id: &str, update: UpdateTodoRequest) -> ServeiceResult<Todo> {
        let id = self.id_generator.parse(id)?;
        let existing_todo = self.repository.get_todo_by_id(&id).await?;
        let updated_todo = TodoModelUpdate::merge(existing_todo, update);

        let todo = self.repository.update_todo(&id, updated_todo).await?;

        self.model_to_domain(todo)
    }

    pub async fn delete_todo(&self, id: &str) -> ServeiceResult<Todo> {
        let id = self.id_generator.parse(id)?;

        let todo = self.repository.delete_todo(&id).await?;

        self.model_to_domain(todo)
    }

    pub async fn search_todo(&self, q: &str) -> ServeiceResult<Vec<Todo>> {
        let todos_model = self.repository.search_todo(q).await?;

        let reasult: Result<Vec<Todo>, ApplicationError> = todos_model
            .into_iter()
            .map(|t| self.model_to_domain(t))
            .collect();

        match reasult {
            Ok(todos) => Ok(todos),
            Err(msg) => Err(msg),
        }
    }

    fn request_to_domain(&self, data: CreateTodoRequest) -> Todo {
        let creation_date = self.clock.now();

        Todo {
            id: self.id_generator.generate(),
            subject: data.subject,
            description: data.description,
            is_done: false,
            due_date: data.due_date,
            created_at: creation_date,
            updated_at: creation_date,
        }
    }

    fn model_to_domain(&self, model: TodoModel) -> Result<Todo, ApplicationError> {
        let id = self
            .id_generator
            .parse(&model.id.id.to_string())
            .map_err(|err| ApplicationError::ServerError(vec![err.to_string()]))?;

        Ok(Todo {
            id,
            subject: model.subject,
            description: model.description,
            is_done: model.is_done,
            due_date: model.due_date,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
    }
}
