use axum::http::StatusCode;

pub async fn request_handler() -> (StatusCode, &'static str) {
    (
        StatusCode::BAD_REQUEST,
        "400 Bad Request: Unhandled endpoint or method.",
    )
}
