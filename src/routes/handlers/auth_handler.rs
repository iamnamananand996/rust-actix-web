use actix_web::{Responder, post, web};
use entity::user;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha256::digest;

use crate::utils::{api_response::ApiResponse, app_state::AppState, jwt::generate_jwt};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

#[post("/register")]
pub async fn register(
    state: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
) -> impl Responder {
    let user = user::ActiveModel {
        name: Set(body.name.clone()),
        email: Set(body.email.clone()),
        password: Set(digest(body.password.clone())),
        ..Default::default()
    };

    let user = user.insert(&state.db).await.unwrap();

    ApiResponse::new(200, format!("User created successfully: {}", user.id), json!(user))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(state: web::Data<AppState>, body: web::Json<LoginRequest>) -> impl Responder {
    let user = user::Entity::find()
        .filter(
            Condition::all()
                .add(user::Column::Email.eq(body.email.clone()))
                .add(user::Column::Password.eq(digest(body.password.clone()))),
        )
        .one(&state.db)
        .await
        .unwrap();

    if user.is_none() {
        return ApiResponse::new(
            401,
            "User not found".to_string(),
            json!("User not found"),
        );
    }

    let user = user.unwrap();

    let token = generate_jwt(user.id, user.email.clone()).unwrap();

    let response = LoginResponse {
        token,
        user: UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        },
    };

    ApiResponse::new(200, "Login successful".to_string(), json!(response))
}




