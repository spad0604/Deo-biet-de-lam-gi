use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, header::AUTHORIZATION},
    response::{IntoResponse, Response, Json},
};
use crate::security::jwt::{verify_jwt, Claims};
use crate::models::api_response::ApiResponse;

pub struct Auth(pub Claims);

impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "mysecret".to_string());

        let auth_header = parts.headers.get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok());
        
        if auth_header.is_none() {
            let response: ApiResponse<()> = ApiResponse {
                status: 401,
                message: "Thiếu header Authorization".to_string(),
                data: None,
            };
            return Err((StatusCode::UNAUTHORIZED, Json(response)).into_response());
        }

        let auth_header = auth_header.unwrap();
        
        if !auth_header.starts_with("Bearer ") {
            let response: ApiResponse<()> = ApiResponse {
                status: 401,
                message: "Sai định dạng token".to_string(),
                data: None,
            };
            return Err((StatusCode::UNAUTHORIZED, Json(response)).into_response());
        }

        let token = &auth_header[7..];
        let decoded = verify_jwt(token, &secret)
            .map_err(|e| {
                tracing::error!("JWT verify failed: {:?}", e);
                let response: ApiResponse<()> = ApiResponse {
                    status: 401,
                    message: "Token không hợp lệ".to_string(),
                    data: None,
                };
                (StatusCode::UNAUTHORIZED, Json(response)).into_response()
            })?;

        Ok(Auth(decoded.claims))
    }
}
