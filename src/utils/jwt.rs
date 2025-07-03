use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::utils::constants::JWT_SECRET;

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub user_id: i32,
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn generate_jwt(user_id: i32, email: String) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = Duration::hours(24);
    let iat = now.timestamp() as usize;

    let claims = JwtClaims {
        user_id,
        exp: (now + exp).timestamp() as usize,
        iat: iat,
        email,
    };

    let secret = JWT_SECRET.as_bytes();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}


pub fn decode_jwt(token: String) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
    let secret = JWT_SECRET.as_bytes();
    let token = decode::<JwtClaims>(&token, &DecodingKey::from_secret(secret), &Validation::default());
    token.map(|token| token.claims)
}
