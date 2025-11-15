use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode, header::AUTHORIZATION},
    response::Json,
    routing::get,
    Router,
};
use sqlx::PgPool;
use crate::db::face_vector_repo::get_all_face_vectors;
use crate::entity::face_vector::FaceVectorResponse;
use crate::models::api_response::ApiResponse;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/api/internal/all-face-vectors", get(all_face_vectors))
}

/// Extractor để validate internal API token
pub(crate) struct InternalAuth;

impl<S> FromRequestParts<S> for InternalAuth
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ApiResponse<()>>);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let internal_token = std::env::var("INTERNAL_API_TOKEN")
            .unwrap_or_else(|_| "your_secret_token_here".to_string());

        let auth_header = parts.headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        match auth_header {
            Some(header) if header.starts_with("Bearer ") => {
                let token = &header[7..];
                if token == internal_token {
                    Ok(InternalAuth)
                } else {
                    Err(ApiResponse::error(StatusCode::UNAUTHORIZED, "Invalid token"))
                }
            }
            _ => {
                Err(ApiResponse::error(StatusCode::UNAUTHORIZED, "Missing or invalid Authorization header"))
            }
        }
    }
}

/// API endpoint để Python Embedded lấy tất cả face vectors
pub async fn all_face_vectors(
    _auth: InternalAuth,
    State(db): State<PgPool>,
) -> (StatusCode, Json<ApiResponse<Vec<FaceVectorResponse>>>) {
    match get_all_face_vectors(&db).await {
        Ok(face_vectors) => {
            ApiResponse::ok("Success", face_vectors)
        }
        Err(e) => {
            tracing::error!("Failed to get face vectors: {:?}", e);
            ApiResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve face vectors",
            )
        }
    }
}

