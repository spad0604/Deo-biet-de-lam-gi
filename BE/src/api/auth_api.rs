use axum::{
    extract::State,
    response::Json,
    routing::post,
    http::StatusCode,
    Router
};

use uuid::Uuid;
use sqlx::PgPool;
use crate::db::user_repo::{register as register_user};
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
    use crate::entity::users::User;

    let user = User {
        id: Uuid::new_v4(),
        password: req.password,
        image_url: "".to_string(), 
        first_name: req.first_name,
        last_name: req.last_name,
        date_of_birth: req.date_of_birth,
        email: req.email,
        phone_number: req.phone_number,
        class: req.class,
        role: req.role,
        created_at: Utc::now(),
    };

    match register_user(&db, user).await {
        Ok(_) => ApiResponse::ok("User registered successfully", ()),
        Err(_) => ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to register user"),
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
        Err(_) => ApiResponse::error(StatusCode::UNAUTHORIZED, "Invalid credentials"),
    }
}
