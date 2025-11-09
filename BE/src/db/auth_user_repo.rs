use sqlx::{Pool, Postgres};
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};
use crate::entity::auth_user::AuthUser;

pub async fn get_user_by_email(db: &Pool<Postgres>, email: &str) -> sqlx::Result<Option<AuthUser>> {
    sqlx::query_as::<_, AuthUser>("SELECT * FROM auth_users WHERE email = $1")
        .bind(email)
        .fetch_optional(db)
        .await
}

pub async fn register(db: &Pool<Postgres>, user: AuthUser) -> sqlx::Result<AuthUser> {
    let id = Uuid::new_v4();

    let hashed = hash(user.password, DEFAULT_COST).unwrap();
    let created = sqlx::query_as::<_, AuthUser>(
        "INSERT INTO auth_users (id, user_id, email, password, role, created_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
        .bind(id)
        .bind(user.user_id)
        .bind(user.email)
        .bind(hashed)
        .bind(user.role)
        .bind(user.created_at)
        .fetch_one(db)
        .await?;

    Ok(created)
}