use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodoRequest {
    #[validate(length(min = 1))]
    #[schema(example = "Buy groceries")]
    pub subject: String,
    #[validate(length(min = 1))]
    #[schema(example = "Buy groceries from the supermarket for the weekend.")]
    pub description: String,
    pub due_date: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodoRequest {
    #[schema(example = "My changed Todo")]
    pub subject: Option<String>,
    #[schema(example = "Keep doing more everyday")]
    pub description: Option<String>,
    #[schema(example = true)]
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
