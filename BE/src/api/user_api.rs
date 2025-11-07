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

use crate::models::users::{User};
