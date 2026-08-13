use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::webserver::AppState;
use tokio::sync::oneshot;

pub enum DisplayCommand {
    GetDisplaysInfo(oneshot::Sender<Vec<DisplayConfigWeb>>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DisplayConfigWeb {
    pub id: i32,
    pub model: String,
    pub display_size: (u16, u16),
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

pub async fn set_display_handler(State(state): State<AppState>) {}
