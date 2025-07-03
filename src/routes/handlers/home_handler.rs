use actix_web::{Responder, get, web};
use sea_orm::EntityTrait;
use serde_json::json;

use crate::utils::{api_response::ApiResponse, app_state::AppState};
use entity::user;

#[get("/{name}")]
pub async fn home(name: web::Path<String>) -> impl Responder {
    ApiResponse::new(200, format!("Hello, {}!", name), name.to_string())
}

#[get("/users/list")]
pub async fn users(state: web::Data<AppState>) -> impl Responder {
    let users = user::Entity::find().all(&state.db).await.unwrap();
    ApiResponse::new(200, format!("Found users {}", users.len()), json!(users))
}
