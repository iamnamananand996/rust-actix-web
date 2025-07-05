use actix_web::{get, put, web};
use chrono::NaiveDate;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder,
};
use serde::{Deserialize, Serialize};

use crate::utils::{api_response::ApiResponse, app_state::AppState};
use entity::user;

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
pub struct UsersResponse {
    pub users: Vec<user::Model>,
    pub pagination: PaginationMeta,
}

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
    query: web::Query<PaginationQuery>,
) -> Result<ApiResponse<UsersResponse>, ApiResponse<String>> {
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
    let mut query_builder = user::Entity::find();

    // Search by name and email
    if let Some(search_term) = &query.search {
        if !search_term.is_empty() {
            let search_condition = Condition::any()
                .add(user::Column::Name.contains(search_term))
                .add(user::Column::Email.contains(search_term));
            query_builder = query_builder.filter(search_condition);
        }
    }

    // Date range filtering
    if let Some(start_date_str) = &query.start_date {
        match NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d") {
            Ok(start_date) => {
                let start_datetime = start_date.and_hms_opt(0, 0, 0).unwrap().and_utc();
                query_builder = query_builder.filter(user::Column::CreatedAt.gte(start_datetime));
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
                query_builder = query_builder.filter(user::Column::CreatedAt.lte(end_datetime));
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
        "name" => {
            query_builder = if sort_order == "asc" {
                query_builder.order_by_asc(user::Column::Name)
            } else {
                query_builder.order_by_desc(user::Column::Name)
            };
        }
        "created_at" => {
            query_builder = if sort_order == "asc" {
                query_builder.order_by_asc(user::Column::CreatedAt)
            } else {
                query_builder.order_by_desc(user::Column::CreatedAt)
            };
        }
        _ => {
            query_builder = query_builder.order_by_desc(user::Column::CreatedAt);
        }
    }

    let paginator = query_builder.paginate(&state.db, per_page);

    let total_items = paginator.num_items().await.map_err(|db_err| {
        ApiResponse::new(500, "Database error".to_string(), db_err.to_string())
    })?;

    let total_pages = paginator.num_pages().await.map_err(|db_err| {
        ApiResponse::new(500, "Database error".to_string(), db_err.to_string())
    })?;

    let users = paginator.fetch_page(page - 1).await.map_err(|db_err| {
        ApiResponse::new(500, "Database error".to_string(), db_err.to_string())
    })?;

    let pagination = PaginationMeta {
        current_page: page,
        per_page,
        total_items,
        total_pages,
    };

    let response = UsersResponse { users, pagination };

    Ok(ApiResponse::new(
        200,
        format!(
            "Users found: {total_items} (page {page} of {total_pages})"
        ),
        response,
    ))
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdatePayload {
    name: String,
    avatar: Option<String>,
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
    user.avatar = Set(body.avatar.clone());

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
