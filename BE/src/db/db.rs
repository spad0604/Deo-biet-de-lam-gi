use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use dotenvy::dotenv;
use std::env;

pub async fn connect_db() -> Pool<Postgres> {
    Ok(dotenv());
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("DB connection failed")
}

