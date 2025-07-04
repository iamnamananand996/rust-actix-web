use actix_web::{body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder, ResponseError};
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

impl<T> std::fmt::Debug for ApiResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiResponse")
            .field("status", &self.status)
            .field("message", &self.message)
            .field("data", &"<data>")
            .finish()
    }
}

impl<T> std::fmt::Display for ApiResponse<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<T> ResponseError for ApiResponse<T>
where
    T: Serialize,
{
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap())
            .content_type("application/json")
            .body(serde_json::to_string(&self).unwrap_or_else(|_| {
                r#"{"status": 500, "message": "Serialization error", "data": null}"#.to_string()
            }))
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