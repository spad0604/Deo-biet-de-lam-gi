use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AuthUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}