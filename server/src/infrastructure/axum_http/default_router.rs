use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn healt_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "message": "OK" }))).into_response()
}

pub async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "message": "Not Found" })),
    )
        .into_response()
}
