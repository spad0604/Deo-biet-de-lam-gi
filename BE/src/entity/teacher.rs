use crate::entity::users::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Teacher {
    pub id: Uuid,
    #[sqlx(flatten)]
    pub user: User,
    pub homeroom_class_id: Option<Uuid>,
}