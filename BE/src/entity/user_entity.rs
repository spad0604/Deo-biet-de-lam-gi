use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserEntity {
    pub id: Uuid,
    pub password: String,
    pub image_url: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: DateTime<Utc>,
    pub email: String,
    pub phone_number: String,
    pub class: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}