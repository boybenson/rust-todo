use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct TodoResponse {
    pub item: String,
    pub is_complete: bool,
}

pub async fn get_todos() -> Json<Vec<TodoResponse>> {
    Json(vec![TodoResponse {
        item: String::from("Learn Rust"),
        is_complete: false,
    }])
}
