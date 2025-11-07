use axum::{
    extract::{State, Multipart},
    response::Json,
    routing::{get, post},
    http::StatusCode,
    Router
};

use sqlx::PgPool;
use serde_json::json;
use crate::db::user_repo::{get_user_by_email, save_image_url};
use crate::security::middle_ware::Auth;
use crate::models::api_response::ApiResponse;
use crate::utils::cloudinary::upload_to_cloudinary;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/api/user/me", get(get_me))
        .route("/api/user/avatar", post(upload_avatar))
}

#[utoipa::path(
    get,
    path = "/api/user/me",
    responses(
        (status = 200, description = "User data retrieved successfully"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "User"
)]
pub async fn get_me(State(db): State<PgPool>, Auth(claims): Auth) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    match get_user_by_email(&db, &claims.sub).await {
        Ok(Some(user)) => ApiResponse::ok("User data retrieved", json!({
            "id": user.id,
            "first_name": user.first_name,
            "last_name": user.last_name,
            "email": user.email,
            "role": user.role,
            "image_url": user.image_url,
            "class": user.class
        })),
        Ok(None) => ApiResponse::error(StatusCode::NOT_FOUND, "User not found"),
        Err(_) => ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
    }
}

#[utoipa::path(
    post,
    path = "/api/user/avatar",
    responses(
        (status = 200, description = "Avatar uploaded successfully"),
        (status = 400, description = "Missing file upload"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "User"
)]
pub async fn upload_avatar(
    State(db): State<PgPool>,
    Auth(claims): Auth,
    mut image: Multipart,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    match get_user_by_email(&db, &claims.sub).await {
        Ok(Some(user)) => {
            while let Some(field) = image.next_field().await.unwrap_or(None) {
                if let Some(file_name_ref) = field.file_name() {
                    let file_name = file_name_ref.to_string();

                    let bytes = field.bytes().await.unwrap_or_default();

                    match upload_to_cloudinary(bytes.to_vec(), &file_name).await {
                        Ok(url) => {
                            if let Err(e) = save_image_url(&db, &url, user.id).await {
                                println!("Error saving image: {:?}", e);
                                return ApiResponse::error(
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    "Không lưu được URL ảnh",
                                );
                            }

                            return ApiResponse::ok(
                                "Upload avatar thành công",
                                json!({ "url": url }),
                            );
                        }
                        Err(e) => {
                            println!("{}", e);
                            return ApiResponse::error(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Lỗi upload Cloudinary",
                            );
                        }
                    }
                }
            }

            ApiResponse::error(StatusCode::BAD_REQUEST, "Thiếu file upload")
        }
        Ok(None) => ApiResponse::error(StatusCode::NOT_FOUND, "User không tồn tại"),
        Err(_) => ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Lỗi truy vấn database"),
    }
}