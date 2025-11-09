use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData, errors::Result};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: String,
    pub email: String
}

pub fn create_jwt(id: &str, role: &str, email: &str, secret: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60))
        .expect("invalid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: id.to_owned(),
        exp: expiration,
        role: role.to_owned(),
        email: email.to_owned(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).expect("Error creating JWT")
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<TokenData<Claims>> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}
