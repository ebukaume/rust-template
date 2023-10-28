use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use super::domain::Todo;

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodoRequest {
    #[validate(length(min = 1))]
    #[schema(example = "Buy groceries")]
    pub subject: String,
    #[validate(length(min = 1))]
    #[schema(example = "Buy groceries from the supermarket for the weekend.")]
    pub description: String,
    #[schema(example = Utc::now)]
    pub due_date: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodoRequest {
    pub subject: Option<String>,
    pub description: Option<String>,
    pub is_done: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Validate, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SearchTodoRequest {
    #[schema(example = "groceries")]
    #[param(example = "todo")]
    #[validate(length(min = 1, message = "is required!"))]
    pub q: String,
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
