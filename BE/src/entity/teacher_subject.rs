use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TeacherSubject {
    pub teacher_id: Uuid,
    pub subject_id: Uuid,
}