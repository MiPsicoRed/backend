use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use std::path::Path;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::{
    adapters::{http::{app_state::AppState, routes::AuthUser}},
    app_error::AppError,
};

pub async fn upload_profile_picture(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let user_id = auth_user.user_id;
    let mut file_url = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::Internal(format!("Error parsing multipart data: {}", e))
    })? {
        if field.name() == Some("file") {
            let content_type = field.content_type().unwrap_or("application/octet-stream");
            let extension = match content_type {
                "image/jpeg" => "jpg",
                "image/png" => "png",
                "image/webp" => "webp",
                _ => return Err(AppError::InvalidPayload),
            };

            let filename = format!("{}.{}", user_id, extension);
            let path_str = format!("uploads/profiles/{}", filename);
            let path = Path::new(&path_str);

            let data = field.bytes().await.map_err(|e| {
                AppError::Internal(format!("Failed to read file data: {}", e))
            })?;

            let mut file = tokio::fs::File::create(&path).await.map_err(|e| {
                AppError::Internal(format!("Failed to create file: {}", e))
            })?;
            file.write_all(&data).await.map_err(|e| {
                AppError::Internal(format!("Failed to write to file: {}", e))
            })?;

            file_url = Some(format!("/api/uploads/profiles/{}", filename));
        }
    }

    if let Some(url) = file_url {
        let parsed_uuid = Uuid::parse_str(&user_id).map_err(|_| AppError::InvalidPayload)?;

        state
            .user_use_cases
            .update_profile_picture_url(&parsed_uuid, &url)
            .await?;

        Ok((StatusCode::OK, url))
    } else {
        Err(AppError::InvalidPayload)
    }
}
