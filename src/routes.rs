use axum::{Router, routing::get};

use crate::handlers::get_todos;

pub fn create_routes() -> Router {
    Router::new().route("/todos", get(get_todos))
}
