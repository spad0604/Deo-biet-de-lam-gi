use uuid::Uuid;
use crate::entity::users::Role;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub uuid: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub role: Role,
    pub image_url: String,
    pub first_name: String,
    pub last_name: String,
    pub class: String
}