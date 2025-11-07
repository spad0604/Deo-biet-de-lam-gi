use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::config::Config;

pub async fn connect_db() -> anyhow::Result<Pool<Postgres>> {
    // Load config from env (uses sensible defaults defined in `Config`)
    let cfg = Config::from_env();
    let db_url = cfg.database_url();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    Ok(pool)
}

