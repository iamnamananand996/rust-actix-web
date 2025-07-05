use actix_web::{get, put, web};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::utils::{api_response::ApiResponse, app_state::AppState};
use entity::user;

#[get("/{id}")]
pub async fn get_user(
    state: web::Data<AppState>,
    name: web::Path<String>,
) -> Result<ApiResponse<user::Model>, ApiResponse<String>> {
    let user_id = match name.to_string().parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return Err(ApiResponse::new(
                400,
                "Invalid user ID format".to_string(),
                "Bad Request".to_string(),
            ));
        }
    };

    let user = user::Entity::find_by_id(user_id).one(&state.db).await;
    match user {
        Ok(Some(user)) => Ok(ApiResponse::new(200, "User found".to_string(), user)),
        Ok(None) => Err(ApiResponse::new(
            404,
            "User not found".to_string(),
            "User not found".to_string(),
        )),
        Err(db_err) => Err(ApiResponse::new(
            500,
            "Database error".to_string(),
            db_err.to_string(),
        )),
    }
}

#[get("/users/list")]
pub async fn users(
    state: web::Data<AppState>,
) -> Result<ApiResponse<Vec<user::Model>>, ApiResponse<String>> {
    let users = user::Entity::find().all(&state.db).await;
    match users {
        Ok(users) => Ok(ApiResponse::new(
            200,
            format!("Found {} users", users.len()),
            users,
        )),
        Err(db_err) => Err(ApiResponse::new(
            500,
            "Database error".to_string(),
            db_err.to_string(),
        )),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdatePayload {
    name: String,
}

#[put("/update/{id}")]
pub async fn update(
    state: web::Data<AppState>,
    body: web::Json<UpdatePayload>,
    id: web::Path<String>,
) -> Result<ApiResponse<user::Model>, ApiResponse<String>> {
    let user_id = match id.to_string().parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return Err(ApiResponse::new(
                400,
                "Invalid user ID format".to_string(),
                "Bad Request".to_string(),
            ));
        }
    };

    let user = user::Entity::find_by_id(user_id).one(&state.db).await;
    let user = match user {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(ApiResponse::new(
                404,
                "User not found".to_string(),
                "User not found".to_string(),
            ));
        }
        Err(db_err) => {
            return Err(ApiResponse::new(
                500,
                "Database error".to_string(),
                db_err.to_string(),
            ));
        }
    };

    let mut user: user::ActiveModel = user.into();
    user.name = Set(body.name.clone());

    let updated_user = user.update(&state.db).await;
    match updated_user {
        Ok(user) => Ok(ApiResponse::new(200, "User updated".to_string(), user)),
        Err(db_err) => Err(ApiResponse::new(
            500,
            "Failed to update user".to_string(),
            db_err.to_string(),
        )),
    }
}
