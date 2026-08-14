use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::{display_thread::DisplayCommand, webserver::AppState};
use tokio::sync::oneshot;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DisplayConfigWeb {
    pub id: i32,
    pub model: String,
    pub display_size: (u16, u16),
    pub current_image: Option<String>,
}

pub async fn handle_display_names(
    State(state): State<AppState>,
) -> Result<Json<Vec<DisplayConfigWeb>>, (StatusCode, String)> {
    let (tx, rx) = oneshot::channel();
    state
        .tx
        .send(DisplayCommand::GetDisplaysInfo(tx))
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let configs = rx
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(configs))
}

pub async fn set_display_handler(
    State(state): State<AppState>,
    Path((id, filename)): Path<(i32, String)>,
) -> Result<(), (StatusCode, String)> {
    state
        .tx
        .send(DisplayCommand::SetDisplay(id, filename))
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok(())
}
