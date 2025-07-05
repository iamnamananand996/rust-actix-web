use crate::utils::jwt::decode_jwt;
use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorInternalServerError,
    middleware::Next,
};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth_header = req.headers().get("Authorization");

    if auth_header.is_none() {
        return Err(ErrorInternalServerError("Unauthorized".to_string()));
    }

    let token = auth_header
        .unwrap()
        .to_str()
        .unwrap()
        .trim_start_matches("Bearer ");

    let token = decode_jwt(token.to_string());

    if token.is_err() {
        return Err(ErrorInternalServerError("Unauthorized".to_string()));
    }

    req.extensions_mut().insert(token.unwrap());

    // let user = User::find_by_id(token.unwrap().claims.sub).await.unwrap();
    next.call(req)
        .await
        .map_err(|_| ErrorInternalServerError("Error".to_string()))
}
