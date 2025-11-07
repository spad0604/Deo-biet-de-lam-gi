use serde::Serialize;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub status: i16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(message: &str, data: T) -> (StatusCode, Json<ApiResponse<T>>) {
        (
            StatusCode::OK,
            Json(ApiResponse {
                status: 200,
                message: message.to_string(),
                data: Some(data),
            }),
        )
    }

    pub fn error(code: StatusCode, message: &str) -> (StatusCode, Json<ApiResponse<T>>) {
        (
            code,
            Json(ApiResponse {
                status: 400,
                message: message.to_string(),
                data: None,
            }),
        )
    }
}