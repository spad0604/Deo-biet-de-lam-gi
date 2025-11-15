use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::entity::face_vector::{FaceVector, FaceVectorResponse};

/// Lưu hoặc cập nhật face vector cho học sinh
pub async fn upsert_face_vector(
    db: &Pool<Postgres>,
    student_id: Uuid,
    embedding: Vec<f32>,
) -> sqlx::Result<FaceVector> {
    let now = chrono::Utc::now();
    
    let result = sqlx::query_as::<_, FaceVector>(
        r#"
        INSERT INTO face_vectors (student_id, embedding, created_at, updated_at)
        VALUES ($1, $2, $3, $3)
        ON CONFLICT (student_id) 
        DO UPDATE SET 
            embedding = $2,
            updated_at = $3
        RETURNING *
        "#
    )
    .bind(student_id)
    .bind(&embedding[..])  // PostgreSQL array từ Vec<f32>
    .bind(now)
    .fetch_one(db)
    .await?;

    Ok(result)
}

/// Lấy face vector theo student_id
pub async fn get_face_vector_by_student_id(
    db: &Pool<Postgres>,
    student_id: Uuid,
) -> sqlx::Result<Option<FaceVector>> {
    let result = sqlx::query_as::<_, FaceVector>(
        "SELECT * FROM face_vectors WHERE student_id = $1"
    )
    .bind(student_id)
    .fetch_optional(db)
    .await?;

    Ok(result)
}

/// Lấy tất cả face vectors (dùng cho Python Embedded sync)
pub async fn get_all_face_vectors(
    db: &Pool<Postgres>,
) -> sqlx::Result<Vec<FaceVectorResponse>> {
    let results = sqlx::query_as::<_, FaceVector>(
        "SELECT * FROM face_vectors"
    )
    .fetch_all(db)
    .await?;

    let responses: Vec<FaceVectorResponse> = results
        .into_iter()
        .map(|fv| FaceVectorResponse {
            id: fv.student_id.to_string(),
            vector: fv.embedding,
        })
        .collect();

    Ok(responses)
}

/// Xóa face vector theo student_id
pub async fn delete_face_vector_by_student_id(
    db: &Pool<Postgres>,
    student_id: Uuid,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM face_vectors WHERE student_id = $1")
        .bind(student_id)
        .execute(db)
        .await?;

    Ok(())
}

