use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime as SurrealDbDateTime, Thing};

use super::request::UpdateTodoRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoModel {
    pub id: Thing,
    pub subject: String,
    pub description: String,
    pub is_done: bool,
    pub due_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct TodoModelUpdate {
    pub subject: String,
    pub description: String,
    pub is_done: bool,
    pub due_date: SurrealDbDateTime,
}

impl TodoModelUpdate {
    pub fn merge(existing: TodoModel, update: UpdateTodoRequest) -> Self {
        Self {
            subject: update.subject.unwrap_or(existing.subject),
            description: update.description.unwrap_or(existing.description),
            is_done: update.is_done.unwrap_or(existing.is_done),
            due_date: SurrealDbDateTime(update.due_date.unwrap_or(existing.due_date)),
        }
    }
}
