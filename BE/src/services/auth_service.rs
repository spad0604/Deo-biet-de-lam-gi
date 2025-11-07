use std::fmt::Display;
use std::io::Empty;
use bcrypt::{verify, hash, DEFAULT_COST};
use crate::security::jwt::create_jwt;
use sqlx::{Pool, Postgres};
use tracing::log::__private_api::log;
use crate::entity::users::User;
use crate::models::api_response::ApiResponse;
use crate::models::login_response::LoginResponse;

pub async fn login_user(db: &Pool<Postgres>, email: &str, password: &str, secret: &str) -> anyhow::Result<LoginResponse> {
    let record = sqlx::query_as::<_, User>(
        "SELECT id, password, image_url, first_name, last_name, date_of_birth, email, phone_number, class, role, created_at FROM users WHERE email = $1"
    )
        .bind(email)
        .fetch_one(db)
        .await?;

    if verify(password, &record.password).unwrap() {
        let jwt_token = create_jwt(email, &record.role.to_string(), secret);
        let login_response = LoginResponse {
            uuid: record.id,
            access_token: jwt_token,
            refresh_token: "".to_string(), 
            role: record.role,
            image_url: record.image_url,
            first_name: record.first_name,
            last_name: record.last_name,
            class: record.class,
        };
        Ok(login_response)
    } else {
        Err(anyhow::anyhow!("Invalid password"))
    }
}