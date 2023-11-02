use chrono::Utc;
use utoipa::{OpenApi, ToSchema};

use super::todos;
use crate::common::error;
use api_documentation::v1::todos as todo_doc;

#[derive(ToSchema)]
#[schema(example = Utc::now, format = "date-time")]
struct DateTime(String);

#[derive(OpenApi)]
#[openapi(
    paths(
        todos::create_todo,
        todos::get_todos,
        todos::get_todo_by_id,
        todos::update_todo,
        todos::delete_todo,
        todos::search_todo
    ),
    components(
        schemas(
            todo_doc::TodoResponse,
            todo_doc::CreateTodoRequest,
            todo_doc::UpdateTodoRequest,
            todo_doc::SearchTodoRequest,
            error::Problem,
            self::DateTime,
        )
    ),
    tags(
        (name = "Todo", description = "Endpoints for manupilating todo resource")
    ),
    info(
        title = "Axum REST API template",
        description = "A simple REST API template",
        version = "v0.1.0",
        contact(name = "Ebuka Umeokonkwo", email = "ebukaume@gmail.com", url = "https://github.com/ebukaume/rust-template"),
        license( name = "MIT", url = "https://opensource.org/license/mit" )
    )
)]
pub struct ApiDoc;
