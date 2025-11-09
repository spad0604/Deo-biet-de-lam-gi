use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: DateTime<Utc>,
    pub phone_number: String,
    pub image_url: String,
    
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "TEXT")]
pub enum Role {
    Teacher,
    Student,
    Admin,
}

impl From<String> for Role {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Teacher" => Role::Teacher,
            "Student" => Role::Student,
            "Admin" => Role::Admin,
            _ => Role::Student, 
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Teacher => write!(f, "Teacher"),
            Role::Student => write!(f, "Student"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}