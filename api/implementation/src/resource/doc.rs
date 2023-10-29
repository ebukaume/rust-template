use chrono::Utc;
use utoipa::{OpenApi, ToSchema};

use super::todo;
use crate::common::error;
use api_documentation::todo as todo_doc;

#[derive(ToSchema)]
#[schema(example = Utc::now, format = "date-time")]
struct DateTime(String);

#[derive(OpenApi)]
#[openapi(
    paths(
        todo::create_todo,
        todo::get_todos,
        todo::get_todo_by_id,
        todo::update_todo,
        todo::delete_todo,
        todo::search_todo
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
        contact(name = "Ebuka Umeokonkwo", email = "ebukaume@gmail.com", url = "http://github.com/ebukaume"),
        license( name = "MIT", url = "https://opensource.org/license/mit" )
    )
)]
pub struct ApiDoc;
