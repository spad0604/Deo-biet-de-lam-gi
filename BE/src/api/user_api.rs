use axum::{
    extract::{State, Multipart},
    response::Json,
    routing::{get, post},
    http::StatusCode,
    Router
};

use sqlx::PgPool;
use serde_json::json;
use crate::db::auth_user_repo::get_user_by_email as get_auth_user_by_email;
use crate::db::classes_repo::find_class_by_id;
use crate::db::teacher_student_repo::{get_teacher_by_id, get_student_by_id, save_teacher_image_url, save_student_image_url};
use crate::db::face_vector_repo::upsert_face_vector;
use crate::entity::users::Role;
use crate::security::middle_ware::Auth;
use crate::models::api_response::ApiResponse;
use crate::utils::cloudinary::upload_to_cloudinary;
use crate::utils::python_api::extract_embedding_from_image;

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
    match get_auth_user_by_email(&db, &claims.email).await {
        Ok(Some(auth_user)) => {
            let role = Role::from(auth_user.role.clone());
            
            let result = match role {
                Role::Teacher => {
                    match get_teacher_by_id(&db, auth_user.user_id).await {
                        Ok(Some(teacher)) =>{
                            let mut class_name : Option<String> = None;

                            if let Some(class_id) = teacher.homeroom_class_id {
                                let class_id_str = class_id.to_string();

                                if let Ok(Some(name)) = find_class_by_id(&db, &class_id_str).await {
                                    class_name = Some(name);
                                }
                            }
                            Some(json!({
                            "id": teacher.id,
                            "first_name": teacher.user.first_name,
                            "last_name": teacher.user.last_name,
                            "email": auth_user.email,
                            "role": auth_user.role,
                            "image_url": teacher.user.image_url,
                            "class_name": class_name
                        }))}
                        _ => None
                    }
                },
                Role::Student => {
                    match get_student_by_id(&db, auth_user.user_id).await {
                        Ok(Some(student)) => {
                            let mut class_name : Option<String> = None;

                            if let Some(class_id) = student.class_id {
                                let class_id_str = class_id.to_string();

                                if let Ok(Some(name)) = find_class_by_id(&db, &class_id_str).await {
                                    class_name = Some(name);
                                }
                            }

                            Some(json!({
                            "id": student.id,
                            "first_name": student.user.first_name,
                            "last_name": student.user.last_name,
                            "email": auth_user.email,
                            "role": auth_user.role,
                            "image_url": student.user.image_url,
                            "class_name": class_name
                        }))},
                        _ => None
                    }
                },
                Role::Admin => Some(json!({
                    "email": auth_user.email,
                    "role": auth_user.role
                }))
            };
            
            match result {
                Some(data) => ApiResponse::ok("User data retrieved", data),
                None => ApiResponse::error(StatusCode::NOT_FOUND, "User not found")
            }
        },
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
    match get_auth_user_by_email(&db, &claims.email).await {
        Ok(Some(auth_user)) => {
            let role = Role::from(auth_user.role.clone());
            
            while let Some(field) = image.next_field().await.unwrap_or(None) {
                if let Some(file_name_ref) = field.file_name() {
                    let file_name = file_name_ref.to_string();
                    let bytes = field.bytes().await.unwrap_or_default();
                    let bytes_clone = bytes.to_vec();

                    match upload_to_cloudinary(bytes.to_vec(), &file_name).await {
                        Ok(url) => {
                            let save_result = match role {
                                Role::Teacher => save_teacher_image_url(&db, &url, auth_user.user_id).await,
                                Role::Student => save_student_image_url(&db, &url, auth_user.user_id).await,
                                Role::Admin => Ok(()) 
                            };
                            
                            if let Err(e) = save_result {
                                println!("Error saving image: {:?}", e);
                                return ApiResponse::error(
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    "Không lưu được URL ảnh",
                                );
                            }

                            if matches!(role, Role::Student) {
                                match extract_embedding_from_image(bytes_clone, &file_name).await {
                                    Ok(embedding) => {
                                        match upsert_face_vector(&db, auth_user.user_id, embedding).await {
                                            Ok(_) => {
                                                println!("Đã lưu face vector cho student {}", auth_user.user_id);
                                            }
                                            Err(e) => {
                                                println!("Lỗi khi lưu face vector: {:?}", e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("Lỗi khi extract embedding từ Python API: {}", e);
                                    }
                                }
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

            return ApiResponse::error(StatusCode::BAD_REQUEST, "Thiếu file upload");
        },
        Ok(None) => return ApiResponse::error(StatusCode::NOT_FOUND, "User không tồn tại"),
        Err(_) => return ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Lỗi truy vấn database"),
    }
}