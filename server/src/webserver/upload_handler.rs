use std::path::Path;

use axum::{extract::Multipart, http::StatusCode};
use tokio::io::AsyncWriteExt;
use tracing::instrument;

use crate::constants::{OG_DIR, ROOT_DIR};

#[instrument]
pub async fn upload_handler(mut multipart: Multipart) -> Result<(), (StatusCode, String)> {
    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        let filename = field.file_name().ok_or((
            StatusCode::BAD_REQUEST,
            "Missing filename in Content-Disposition header".to_owned(),
        ))?;
        let mut f = tokio::fs::File::create_new(Path::new(&*ROOT_DIR).join(OG_DIR).join(filename))
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
        {
            f.write(&chunk)
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
            println!("received {} bytes", chunk.len());
        }
    }

    Ok(())
}
