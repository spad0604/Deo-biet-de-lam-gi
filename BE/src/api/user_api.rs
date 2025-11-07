use axum::{
    extract::{State, Path},
    response::Json,
    routing::{get, post, delete},
    http::StatusCode,
    Router
};

use uuid::Uuid;
use sqlx::PgPool;
use serde_json::json;
use crate::db::user_repo::{get_user_by_id, register as register_user, get_user_by_email};
use crate::security::middle_ware::Auth;
use crate::services::auth_service::login_user;
use crate::models::register_request::{RegisterRequest, LoginRequest};
use crate::models::login_response::LoginResponse;
use crate::models::api_response::ApiResponse;
use chrono::Utc;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/me", get(get_me))
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn get_me(State(db): State<PgPool>, Auth(claims): Auth) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
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

async fn register(State(db): State<PgPool>, Json(req): Json<RegisterRequest>) -> (StatusCode, Json<ApiResponse<()>>) {
    use crate::entity::users::User;
    use chrono::Utc;

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

async fn login(State(db): State<PgPool>, Json(req): Json<LoginRequest>) -> (StatusCode, Json<ApiResponse<LoginResponse>>) {
    const SECRET: &str = "your-secret-key";
    
    match login_user(&db, &req.email, &req.password, SECRET).await {
        Ok(response) => ApiResponse::ok("Login successful", response),
        Err(_) => ApiResponse::error(StatusCode::UNAUTHORIZED, "Invalid credentials"),
    }
}