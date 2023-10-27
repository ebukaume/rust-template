use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::domain::Todo;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodoRequest {
    #[validate(length(min = 1))]
    pub subject: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub due_date: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct UpdateTodoRequest {
    pub subject: Option<String>,
    pub description: Option<String>,
    pub is_done: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
}

impl UpdateTodoRequest {
    pub fn merge(mut self, original: Todo) -> Self {
        self.subject = self.subject.or(Some(original.subject));
        self.description = self.description.or(Some(original.description));
        self.due_date = self.due_date.or(Some(original.due_date));
        self.is_done = self.is_done.or(Some(original.is_done));

        self
    }
}
