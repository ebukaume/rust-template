use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::Todo;

#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TodoResponse {
    #[schema(example = "01HDWDYAF9NWCR985TDBZYCDN8")]
    pub id: String,
    #[schema(example = "My first Todo")]
    pub subject: String,
    #[schema(example = "Do more today")]
    pub description: String,
    #[schema(example = false)]
    pub is_done: bool,
    pub due_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for TodoResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl From<Todo> for TodoResponse {
    fn from(value: Todo) -> Self {
        Self {
            id: value.id.to_string(),
            subject: value.subject,
            description: value.description,
            is_done: value.is_done,
            due_date: value.due_date,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
