use crate::utils::{api_response::ApiResponse, app_state::AppState, jwt::JwtClaims};
use actix_web::{delete, get, post, put, web};
use chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePostRequest {
    pub user_id: String, // Accept as string from JSON
    pub title: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub text: String,
}

#[get("/{id}")]
pub async fn get_post(
    state: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<ApiResponse<entity::post::Model>, ApiResponse<String>> {
    let post_id = id
        .to_string()
        .parse::<i32>()
        .map_err(|_| ApiResponse::new(400, "Invalid post ID format".to_string(), "".to_string()))?;

    let post = entity::post::Entity::find_by_id(post_id)
        .one(&state.db)
        .await
        .map_err(|_| ApiResponse::new(500, "Database error".to_string(), "".to_string()))?;

    match post {
        Some(post) => Ok(ApiResponse::new(200, "Post found".to_string(), post)),
        None => Err(ApiResponse::new(
            404,
            "Post not found".to_string(),
            "".to_string(),
        )),
    }
}

#[get("/posts/list")]
pub async fn posts(
    state: web::Data<AppState>,
) -> Result<ApiResponse<Vec<entity::post::Model>>, ApiResponse<String>> {
    let posts = entity::post::Entity::find().all(&state.db).await;
    match posts {
        Ok(posts) => Ok(ApiResponse::new(
            200,
            format!("Posts found: {}", posts.len()),
            posts,
        )),
        Err(_) => Err(ApiResponse::new(
            500,
            "Database error".to_string(),
            "".to_string(),
        )),
    }
}

#[post("/create")]
pub async fn create_post(
    state: web::Data<AppState>,
    body: web::Json<CreatePostRequest>,
) -> Result<ApiResponse<entity::post::Model>, ApiResponse<String>> {
    // Parse user_id from string to integer
    let user_id = body
        .user_id
        .parse::<i32>()
        .map_err(|_| ApiResponse::new(400, "Invalid user_id format".to_string(), "".to_string()))?;

    let new_post = entity::post::ActiveModel {
        user_id: Set(user_id),
        title: Set(body.title.clone()),
        text: Set(body.text.clone()),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

    let post = new_post
        .insert(&state.db)
        .await
        .map_err(|_| ApiResponse::new(500, "Database error".to_string(), "".to_string()))?;
    Ok(ApiResponse::new(200, "Post created".to_string(), post))
}

#[put("/update/{id}")]
pub async fn update_post(
    state: web::Data<AppState>,
    body: web::Json<UpdatePostRequest>,
    id: web::Path<String>,
) -> Result<ApiResponse<entity::post::Model>, ApiResponse<String>> {
    let post_id = id
        .to_string()
        .parse::<i32>()
        .map_err(|_| ApiResponse::new(400, "Invalid post ID format".to_string(), "".to_string()))?;

    let post = entity::post::Entity::find_by_id(post_id)
        .one(&state.db)
        .await
        .map_err(|_| ApiResponse::new(500, "Database error".to_string(), "".to_string()))?;

    match post {
        Some(post) => {
            let mut post_active: entity::post::ActiveModel = post.into();
            post_active.title = Set(body.title.clone());
            post_active.text = Set(body.text.clone());
            post_active.updated_at = Set(chrono::Utc::now().naive_utc());

            let updated_post = post_active
                .update(&state.db)
                .await
                .map_err(|_| ApiResponse::new(500, "Database error".to_string(), "".to_string()))?;

            Ok(ApiResponse::new(
                200,
                "Post updated".to_string(),
                updated_post,
            ))
        }
        None => Err(ApiResponse::new(
            404,
            "Post not found".to_string(),
            "".to_string(),
        )),
    }
}

#[delete("/delete/{id}")]
pub async fn delete_post(
    state: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<ApiResponse<entity::post::Model>, ApiResponse<String>> {
    let post_id = id
        .to_string()
        .parse::<i32>()
        .map_err(|_| ApiResponse::new(400, "Invalid post ID format".to_string(), "".to_string()))?;

    let post = entity::post::Entity::find_by_id(post_id)
        .one(&state.db)
        .await
        .map_err(|_| ApiResponse::new(500, "Database error".to_string(), "".to_string()))?;

    match post {
        Some(post) => {
            let post_clone = post.clone();
            let post_active: entity::post::ActiveModel = post.into();
            post_active
                .delete(&state.db)
                .await
                .map_err(|_| ApiResponse::new(500, "Database error".to_string(), "".to_string()))?;

            Ok(ApiResponse::new(
                200,
                "Post deleted".to_string(),
                post_clone,
            ))
        }
        None => Err(ApiResponse::new(
            404,
            "Post not found".to_string(),
            "".to_string(),
        )),
    }
}



#[get("/posts/my-posts")]
pub async fn get_posts_by_user(
    state: web::Data<AppState>,
    claims: JwtClaims,
) -> Result<ApiResponse<Vec<entity::post::Model>>, ApiResponse<String>> {
    let user_id = claims.user_id;
    
    let posts_result = entity::post::Entity::find()
        .filter(entity::post::Column::UserId.eq(user_id))
        .all(&state.db)
        .await;
    
    match posts_result {
        Ok(user_posts) => Ok(ApiResponse::new(
            200,
            format!("Posts found: {}", user_posts.len()),
            user_posts,
        )),
        Err(_) => Err(ApiResponse::new(
            500,
            "Database error".to_string(),
            "".to_string(),
        )),
    }
}