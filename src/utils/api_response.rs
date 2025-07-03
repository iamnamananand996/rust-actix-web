use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::StatusCode, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(status: u16, message: String, data: T) -> Self {
        Self {
            status,
            message,
            data,
        }
    }
}

impl<T> Responder for ApiResponse<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let json_body = serde_json::to_string(&self).unwrap_or_else(|_| {
            r#"{"status": 500, "message": "Serialization error", "data": null}"#.to_string()
        });

        HttpResponse::build(StatusCode::from_u16(self.status).unwrap())
            .content_type("application/json")
            .body(json_body)
    }
}