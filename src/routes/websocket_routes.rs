use actix_web::{HttpResponse, Result, web};

use super::handlers::websocket_handler::websocket_handler;

/// Configure websocket routes
pub fn websocket_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws").route(web::get().to(websocket_handler)));
}

/// Health check endpoint for websocket service
pub async fn websocket_health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "WebSocket service is running",
        "endpoint": "/ws"
    })))
}

/// Configure websocket API routes with health check
pub fn websocket_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/websocket").route("/health", web::get().to(websocket_health)));
}
