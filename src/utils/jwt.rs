use std::future::{self, Ready};

use crate::utils::constants::JWT_SECRET;
use actix_web::{dev::Payload, error::ErrorBadRequest, FromRequest, HttpRequest, Error, HttpMessage};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub user_id: i32,
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

impl FromRequest for JwtClaims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Ready<Result<JwtClaims, Error>> {
        match req.extensions().get::<JwtClaims>() {
            Some(claims) => future::ready(Ok(claims.clone())),
            None => future::ready(Err(ErrorBadRequest("Unauthorized".to_string()))),
        }
    }
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
    let token = decode::<JwtClaims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    );
    token.map(|token| token.claims)
}
