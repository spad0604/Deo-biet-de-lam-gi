use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::entity::users::Role;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: DateTime<Utc>,
    pub phone_number: String,
    pub class: String,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}