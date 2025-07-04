use actix_web::{post, web};
use entity::user;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
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
) -> Result<ApiResponse<user::Model>, ApiResponse<String>> {
    let user = user::ActiveModel {
        name: Set(body.name.clone()),
        email: Set(body.email.clone()),
        password: Set(digest(body.password.clone())),
        ..Default::default()
    };

    let user = user.insert(&state.db).await;

    match user {
        Ok(user) => Ok(ApiResponse::new(
            200,
            format!("User created successfully: {}", user.id),
            user,
        )),
        Err(db_err) => Err(ApiResponse::new(500, "Failed to create user".to_string(), db_err.to_string())),
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(state: web::Data<AppState>, body: web::Json<LoginRequest>) -> Result<ApiResponse<LoginResponse>, ApiResponse<String>> {
    let user = user::Entity::find()
        .filter(
            Condition::all()
                .add(user::Column::Email.eq(body.email.clone()))
                .add(user::Column::Password.eq(digest(body.password.clone()))),
        )
        .one(&state.db)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(db_err) => return Err(ApiResponse::new(500, "Database error".to_string(), db_err.to_string())),
    };

    println!("user: {:?}, password: {:?}, email: {:?}", user, digest(body.password.clone()), body.email);

    if user.is_none() {
        return Err(ApiResponse::new(401, "User not found".to_string(), "User not found".to_string()));   
    }

    let user = user.unwrap();

    let token = generate_jwt(user.id, user.email.clone()).expect("Failed to generate JWT");

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

    Ok(ApiResponse::new(200, "Login successful".to_string(), response))
}
