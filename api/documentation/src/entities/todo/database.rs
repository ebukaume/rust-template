use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime as SurrealDbDateTime, Thing};

use super::request::UpdateTodoRequest as ApiUpdateTodoRequest;

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
pub struct UpdateTodoRequest {
    pub subject: Option<String>,
    pub description: Option<String>,
    pub is_done: Option<bool>,
    pub due_date: Option<SurrealDbDateTime>,
}

impl From<ApiUpdateTodoRequest> for UpdateTodoRequest {
    fn from(value: ApiUpdateTodoRequest) -> Self {
        Self {
            subject: value.subject,
            description: value.description,
            is_done: value.is_done,
            due_date: value.due_date.map(SurrealDbDateTime),
        }
    }
}
