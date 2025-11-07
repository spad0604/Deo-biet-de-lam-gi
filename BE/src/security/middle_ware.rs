use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, header::AUTHORIZATION},
};
use crate::security::jwt::{verify_jwt, Claims};

pub struct Auth(pub Claims);

impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "mysecret".to_string());

        let auth_header = parts.headers.get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Thiếu header Authorization".to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Sai định dạng token".to_string()));
        }

        let token = &auth_header[7..];
        let decoded = verify_jwt(token, &secret)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Token không hợp lệ".to_string()))?;

        Ok(Auth(decoded.claims))
    }
}
