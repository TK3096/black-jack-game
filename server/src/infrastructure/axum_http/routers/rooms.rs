use axum::{
    Router,
    response::IntoResponse,
    routing::{get, post},
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(list_room))
        .route("/:id/join", post(join_room))
        .route("/:id/ws", get(handle_ws))
}

pub async fn list_room() -> impl IntoResponse {}

pub async fn join_room() -> impl IntoResponse {}

pub async fn handle_ws() -> impl IntoResponse {}
