use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;

pub async fn find_class_by_id(db: &Pool<Postgres>, class_id: &str) -> Result<Option<String>, Error> {

    let Ok(uuid) = Uuid::parse_str(class_id) else {
        return Ok(None);
    };

    sqlx::query_scalar(
                        "SELECT name FROM classes WHERE id=$1",
    )
        .bind(uuid)
        .fetch_optional(db)
        .await
}