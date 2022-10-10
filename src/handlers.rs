use std::sync::Arc;

use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};

use crate::repositories::*;

pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repositoy): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repositoy.create(payload);

    (StatusCode::CREATED, Json(todo))
}
