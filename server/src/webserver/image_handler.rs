use std::path::PathBuf;

use axum::{
    Json,
    extract::Path,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use tracing::instrument;

use crate::constants::{OG_DIR, ROOT_DIR};

#[instrument]
pub async fn handle_image(Path(filename): Path<String>) -> impl IntoResponse {
    let path_buf = PathBuf::from(&filename);
    let safe_filename = match path_buf.file_name() {
        Some(name) => name,
        None => return (StatusCode::BAD_REQUEST, "Invalid filename").into_response(),
    };

    let image_path = std::path::Path::new(&*ROOT_DIR)
        .join(OG_DIR)
        .join(safe_filename);

    match tokio::fs::read(&image_path).await {
        Ok(bytes) => {
            let mime_type = mime_guess::from_path(&image_path)
                .first_or_octet_stream()
                .to_string();

            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, mime_type.parse().unwrap());
            headers.insert(
                header::CACHE_CONTROL,
                "public, max-age=31536000, immutable".parse().unwrap(),
            );

            (StatusCode::OK, headers, bytes).into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, "Image not found").into_response(),
    }
}

#[instrument]
pub async fn handle_image_names() -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let files_dir = std::path::Path::new(&*ROOT_DIR).join(OG_DIR);
    let mut entries = tokio::fs::read_dir(files_dir)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let mut filenames = Vec::new();

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
    {
        if entry
            .file_type()
            .await
            .map(|ft| ft.is_file())
            .unwrap_or(false)
        {
            if let Some(name) = entry.file_name().to_str() {
                filenames.push(name.to_string());
            }
        }
    }

    Ok(Json(filenames))
}
