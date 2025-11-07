use serde::Serialize;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T: Serialize + ToSchema> {
    pub status: i16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize + ToSchema> ApiResponse<T> {
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