use sqlx::{Pool, Postgres};
use uuid::Uuid;
use bcrypt::{verify, hash, DEFAULT_COST};
use crate::entity::users::User;

pub async fn get_all(db: &Pool<Postgres>) -> sqlx::Result<Vec<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(db)
        .await
}

pub async fn get_user_by_id(db: &Pool<Postgres>, id: Uuid) -> sqlx::Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(db)
        .await
}

pub async fn get_user_by_email(db: &Pool<Postgres>, email: &str) -> sqlx::Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(db)
        .await
}

pub async fn save_image_url(db: &Pool<Postgres>, image_url: &str, id: &str) -> sqlx::Result<()> {
    sqlx::query("UPDATE users SET image_url = $1 WHERE id = $2")
        .bind(image_url)
        .bind(id)
        .fetch_one(db)
        .await?;

    Ok(())
}

pub async fn register(db: &Pool<Postgres>, user: User) -> sqlx::Result<User> {
    let id = Uuid::new_v4();

    let hashed = hash(user.password, DEFAULT_COST).unwrap();
    let created = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, password, first_name, last_name, email, date_of_birth, phone_number, class, role, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
         RETURNING *"
    )
        .bind(id)
        .bind(hashed)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.email)
        .bind(user.date_of_birth)
        .bind(user.phone_number)
        .bind(user.class)
        .bind(user.role)
        .bind(user.created_at)
        .fetch_one(db)
        .await?;

    Ok(created)
}

pub async fn delete(db: &Pool<Postgres>, id: Uuid) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn edit(db: &Pool<Postgres>, user: User, id: Uuid) -> sqlx::Result<Option<User>> {
    let valid_user = get_user_by_id(db, id).await?;

    if valid_user.is_none() {
        return Ok(None);
    }

    let updated = sqlx::query_as::<_, User>(
        "UPDATE users
         SET first_name = $1,
             last_name = $2,
             email = $3,
             date_of_birth = $4,
             phone_number = $5,
             class = $6,
             role = $7
         WHERE id = $8
         RETURNING *"
    )
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.email)
        .bind(user.date_of_birth)
        .bind(user.phone_number)
        .bind(user.class)
        .bind(user.role)
        .bind(id)
        .fetch_optional(db)
        .await?;

    Ok(updated)
}