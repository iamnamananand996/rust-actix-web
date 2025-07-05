use crate::utils::{
    api_response::ApiResponse,
    app_state::AppState,
    constants::{AWS_REGION, S3_BUCKET_NAME},
};
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{post, web};
use aws_sdk_s3::primitives::ByteStream;
use serde::{Deserialize, Serialize};
use std::io::Read;
use uuid::Uuid;

#[derive(MultipartForm)]
pub struct FileUploadForm {
    #[multipart(rename = "file")]
    pub file: TempFile,
}

#[derive(Deserialize, Serialize)]
pub struct FileUploadResponse {
    pub message: String,
    pub file_url: String,
    pub file_key: String,
}

#[post("/upload")]
pub async fn upload_file(
    state: web::Data<AppState>,
    MultipartForm(form): MultipartForm<FileUploadForm>,
) -> Result<ApiResponse<FileUploadResponse>, ApiResponse<String>> {
    // Get file info
    let file_name = form.file.file_name.as_ref().ok_or_else(|| {
        ApiResponse::new(400, "File name is required".to_string(), "".to_string())
    })?;

    let content_type = form
        .file
        .content_type
        .as_ref()
        .map(|ct| ct.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    // Generate unique file key
    let file_extension = std::path::Path::new(file_name)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("bin");

    let file_key = format!("uploads/{}.{}", Uuid::new_v4(), file_extension);

    // Read file content
    let file_path = form.file.file.path();
    let mut file_content = Vec::new();
    let mut file = match std::fs::File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(ApiResponse::new(
                500,
                format!("Failed to read file: {}", e),
                "".to_string(),
            ));
        }
    };

    if let Err(e) = file.read_to_end(&mut file_content) {
        return Err(ApiResponse::new(
            500,
            format!("Failed to read file content: {}", e),
            "".to_string(),
        ));
    }

    // Upload to S3
    let body = ByteStream::from(file_content);

    let upload_result = state
        .s3_client
        .put_object()
        .bucket(S3_BUCKET_NAME.clone())
        .key(&file_key)
        .body(body)
        .content_type(&content_type)
        .send()
        .await;

    match upload_result {
        Ok(_) => {
            let file_url = format!(
                "https://{}.s3.{}.amazonaws.com/{}",
                S3_BUCKET_NAME.clone(),
                AWS_REGION.clone(),
                file_key
            );

            let response = FileUploadResponse {
                message: "File uploaded successfully".to_string(),
                file_url,
                file_key,
            };

            Ok(ApiResponse::new(
                200,
                "File uploaded successfully".to_string(),
                response,
            ))
        }
        Err(e) => Err(ApiResponse::new(
            500,
            format!("S3 upload failed: {}", e),
            "".to_string(),
        )),
    }
}
