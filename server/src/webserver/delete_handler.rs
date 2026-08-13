use axum::{extract::Path, http::StatusCode};

use crate::constants::{OG_DIR, RAW_DIR, ROOT_DIR};

pub async fn delete_handler(
    Path(query_filename): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let og_dir = std::path::Path::new(&*ROOT_DIR).join(OG_DIR);
    let raw_dir = std::path::Path::new(&*ROOT_DIR).join(RAW_DIR);
    let mut deleted_once = false;

    let mut dir_reader = tokio::fs::read_dir(og_dir)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    while let Some(f) = dir_reader
        .next_entry()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
    {
        if *f.file_name() == *query_filename {
            tokio::fs::remove_file(f.path())
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
            deleted_once = true;
        }
    }
    let mut dir_reader = tokio::fs::read_dir(raw_dir)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    while let Some(f) = dir_reader
        .next_entry()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
    {
        let i = query_filename
            .match_indices('.')
            .last()
            .ok_or((
                StatusCode::BAD_REQUEST,
                "A filename didn't contain a dot".to_owned(),
            ))?
            .0;
        let ext = query_filename.get((i + 1)..).ok_or((
            StatusCode::BAD_REQUEST,
            "File extension slice failed ".to_owned(),
        ))?;
        let file_prefix = query_filename.get(..i).ok_or((
            StatusCode::BAD_REQUEST,
            "File prefix slice failed ".to_owned(),
        ))?;
        let ext = if ext == "gif" { "gif" } else { "raw" };
        let disk_filename = f.file_name().into_string().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Conversion of OsString to str failed".to_owned(),
            )
        })?;
        if disk_filename.starts_with(file_prefix) && disk_filename.ends_with(ext) {
            tokio::fs::remove_file(f.path())
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
            deleted_once = true;
        }
    }

    if deleted_once {
        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
