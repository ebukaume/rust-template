use std::vec;

use axum::{
    extract::rejection::{FormRejection, JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use surrealdb::Error as SurrealDBError;
use tracing::{error, warn};
use utoipa::ToSchema;
use validator::ValidationErrors;

use crate::util::ValidationError;

#[derive(Serialize, Debug, ToSchema)]
pub struct Problem<'a> {
    pub code: &'a str,
    pub issues: Vec<String>,
}

impl<'a> Problem<'a> {
    fn new(code: &'a str, issues: Vec<String>) -> Self {
        Self { code, issues }
    }
}

#[derive(Debug)]
pub enum ApplicationError {
    ValidationError(Vec<String>),
    ServerError(Vec<String>),
    NotFound(String),
}

pub enum RepositoryError {
    Connection(String),
    Query(String),
    NotFound(String),
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        match self {
            ApplicationError::ValidationError(issues) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(Problem::new("VALIDATION_ERROR", issues)),
                )
                    .into_response()
            }
            ApplicationError::ServerError(issues) => {
                error!("{:?}", Problem::new("SERVER_ERROR", issues));

                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Problem::new(
                        "SERVER_ERROR",
                        vec![String::from("This is on us, we will take care of it.")],
                    )),
                )
                    .into_response();
            }
            ApplicationError::NotFound(resource_id) => {
                let message = vec![format!("resource with id {} does not exist!", resource_id)];
                let problem = Problem::new("RESOURCE_NOT_FOUND", message);
                warn!("{:?}", problem);

                (StatusCode::NOT_FOUND, Json(problem)).into_response()
            }
        }
    }
}

impl From<FormRejection> for ApplicationError {
    fn from(value: FormRejection) -> Self {
        Self::ValidationError(vec![value.body_text()])
    }
}

impl From<PathRejection> for ApplicationError {
    fn from(value: PathRejection) -> Self {
        Self::ValidationError(vec![value.body_text()])
    }
}

impl From<QueryRejection> for ApplicationError {
    fn from(value: QueryRejection) -> Self {
        Self::ValidationError(vec![value.body_text()])
    }
}

impl From<JsonRejection> for ApplicationError {
    fn from(value: JsonRejection) -> Self {
        Self::ValidationError(vec![value.body_text()])
    }
}

impl From<ValidationErrors> for ApplicationError {
    fn from(value: ValidationErrors) -> Self {
        Self::ValidationError(
            value
                .to_string()
                .split('\n')
                .map(ToString::to_string)
                .collect(),
        )
    }
}

impl From<SurrealDBError> for RepositoryError {
    fn from(value: SurrealDBError) -> Self {
        match value {
            SurrealDBError::Db(err) => {
                dbg!(&err);
                RepositoryError::Query(err.to_string())
            }
            SurrealDBError::Api(err) => {
                dbg!(&err);

                RepositoryError::Connection(err.to_string())
            }
        }
    }
}

impl From<RepositoryError> for ApplicationError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::Connection(err) => ApplicationError::ServerError(vec![err]),
            RepositoryError::Query(err) => ApplicationError::ServerError(vec![err]),
            RepositoryError::NotFound(resource_id) => ApplicationError::NotFound(resource_id),
        }
    }
}

impl From<ValidationError> for ApplicationError {
    fn from(value: ValidationError) -> Self {
        match value {
            ValidationError::InvalidUlid(id) => ApplicationError::NotFound(id),
        }
    }
}
