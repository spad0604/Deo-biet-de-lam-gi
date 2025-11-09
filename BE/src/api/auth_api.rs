use axum::{
    extract::State,
    response::Json,
    routing::post,
    http::StatusCode,
    Router
};

use uuid::Uuid;
use sqlx::PgPool;
use crate::db::auth_user_repo::{register as register_auth_user};
use crate::db::teacher_student_repo::{create_teacher, create_student};
use crate::services::auth_service::login_user;
use crate::models::register_request::{RegisterRequest, LoginRequest};
use crate::models::login_response::LoginResponse;
use crate::models::api_response::ApiResponse;
use chrono::Utc;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully"),
        (status = 500, description = "Failed to register user")
    ),
    tag = "Authentication"
)]
pub async fn register(State(db): State<PgPool>, Json(req): Json<RegisterRequest>) -> (StatusCode, Json<ApiResponse<()>>) {
    use crate::entity::auth_user::AuthUser;

    let user_id = Uuid::new_v4();
    let auth_user = AuthUser {
        id: Uuid::new_v4(),
        user_id,
        email: req.email.clone(),
        password: req.password,
        role: req.role.to_string(),
        created_at: Utc::now(),
    };

    match register_auth_user(&db, auth_user).await {
        Ok(_) => {
            use crate::entity::user_entities::{TeacherEntity, StudentEntity};
            use crate::entity::users::Role;

            match req.role {
                Role::Teacher => {
                    let teacher = TeacherEntity {
                        id: user_id,
                        first_name: req.first_name.clone(),
                        last_name: req.last_name.clone(),
                        date_of_birth: req.date_of_birth,
                        phone_number: req.phone_number.clone(),
                        image_url: "".to_string(),
                        homeroom_class_id: None,
                    };

                    if let Err(e) = create_teacher(&db, teacher).await {
                        println!("{}", e);
                        return ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create teacher record");
                    }
                }
                Role::Student => {
                    let student = StudentEntity {
                        id: user_id,
                        first_name: req.first_name.clone(),
                        last_name: req.last_name.clone(),
                        date_of_birth: req.date_of_birth,
                        phone_number: req.phone_number.clone(),
                        image_url: "".to_string(),
                        class_id: Uuid::new_v4(), 
                    };

                    if let Err(_) = create_student(&db, student).await {
                        return ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create student record");
                    }
                }
                Role::Admin => {
                }
            }

            ApiResponse::ok("User registered successfully", ())
        },
        Err(e) => {
            println!("{}", e);
            ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to register user")
        },
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "Authentication"
)]
pub async fn login(State(db): State<PgPool>, Json(req): Json<LoginRequest>) -> (StatusCode, Json<ApiResponse<LoginResponse>>) {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "mysecret".to_string());
    
    match login_user(&db, &req.email, &req.password, &secret).await {
        Ok(response) => ApiResponse::ok("Login successful", response),
        Err(e) => {
            println!("{}", e);
            ApiResponse::error(StatusCode::UNAUTHORIZED, "Invalid credentials")
        },
    }
}
