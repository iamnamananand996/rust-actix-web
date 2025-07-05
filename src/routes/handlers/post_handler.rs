use crate::utils::{api_response::ApiResponse, app_state::AppState, jwt::JwtClaims};
use actix_web::{delete, get, post, put, web};
use chrono::{self, NaiveDate};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePostRequest {
    pub user_id: String, // Accept as string from JSON
    pub title: String,
    pub text: String,
    pub banner: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub text: String,
    pub banner: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub current_page: u64,
    pub per_page: u64,
    pub total_items: u64,
    pub total_pages: u64,
}

#[derive(Debug, Serialize)]
pub struct PostsResponse {
    pub posts: Vec<entity::post::Model>,
    pub pagination: PaginationMeta,
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
    query: web::Query<PaginationQuery>,
) -> Result<ApiResponse<PostsResponse>, ApiResponse<String>> {
    let page = query.page.unwrap_or(1);
    let per_page = query.limit.unwrap_or(10);

    // Ensure page is at least 1
    let page = if page < 1 { 1 } else { page };

    // Limit per_page to reasonable bounds
    let per_page = if per_page > 100 {
        100
    } else if per_page < 1 {
        10
    } else {
        per_page
    };

    // Build query with filters
    let mut query_builder = entity::post::Entity::find();

    // Search by title and text
    if let Some(search_term) = &query.search {
        if !search_term.is_empty() {
            let search_condition =
                Condition::any().add(entity::post::Column::Title.contains(search_term));
            query_builder = query_builder.filter(search_condition);
        }
    }

    // Date range filtering
    if let Some(start_date_str) = &query.start_date {
        match NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d") {
            Ok(start_date) => {
                let start_datetime = start_date.and_hms_opt(0, 0, 0).unwrap().and_utc();
                query_builder =
                    query_builder.filter(entity::post::Column::CreatedAt.gte(start_datetime));
            }
            Err(_) => {
                return Err(ApiResponse::new(
                    400,
                    "Invalid start_date format. Use YYYY-MM-DD".to_string(),
                    "Bad Request".to_string(),
                ));
            }
        }
    }

    if let Some(end_date_str) = &query.end_date {
        match NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d") {
            Ok(end_date) => {
                let end_datetime = end_date.and_hms_opt(23, 59, 59).unwrap().and_utc();
                query_builder =
                    query_builder.filter(entity::post::Column::CreatedAt.lte(end_datetime));
            }
            Err(_) => {
                return Err(ApiResponse::new(
                    400,
                    "Invalid end_date format. Use YYYY-MM-DD".to_string(),
                    "Bad Request".to_string(),
                ));
            }
        }
    }

    // Sorting
    let sort_by = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");

    match sort_by {
        "title" => {
            query_builder = if sort_order == "asc" {
                query_builder.order_by_asc(entity::post::Column::Title)
            } else {
                query_builder.order_by_desc(entity::post::Column::Title)
            };
        }
        "created_at" => {
            query_builder = if sort_order == "asc" {
                query_builder.order_by_asc(entity::post::Column::CreatedAt)
            } else {
                query_builder.order_by_desc(entity::post::Column::CreatedAt)
            };
        }
        _ => {
            query_builder = query_builder.order_by_desc(entity::post::Column::CreatedAt);
        }
    }

    let paginator = query_builder.paginate(&state.db, per_page);

    let total_items = paginator.num_items().await.map_err(|db_err| {
        ApiResponse::new(500, "Database error".to_string(), db_err.to_string())
    })?;

    let total_pages = paginator.num_pages().await.map_err(|db_err| {
        ApiResponse::new(500, "Database error".to_string(), db_err.to_string())
    })?;

    let posts = paginator.fetch_page(page - 1).await.map_err(|db_err| {
        ApiResponse::new(500, "Database error".to_string(), db_err.to_string())
    })?;

    let pagination = PaginationMeta {
        current_page: page,
        per_page,
        total_items,
        total_pages,
    };

    let response = PostsResponse { posts, pagination };

    Ok(ApiResponse::new(
        200,
        format!(
            "Posts found: {} (page {} of {})",
            total_items, page, total_pages
        ),
        response,
    ))
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
        banner: Set(body.banner.clone()),
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
            post_active.banner = Set(body.banner.clone());

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
