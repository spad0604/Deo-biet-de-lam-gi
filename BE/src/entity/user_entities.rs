use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TeacherEntity {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: DateTime<Utc>,
    pub phone_number: String,
    pub image_url: String,
    pub homeroom_class_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct StudentEntity {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: DateTime<Utc>,
    pub phone_number: String,
    pub image_url: String,
    pub class_id: Uuid,
}