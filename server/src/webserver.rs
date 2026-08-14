mod delete_handler;
mod display_handler;
mod image_handler;
mod static_handler;
mod upload_handler;

pub use display_handler::DisplayConfigWeb;
use std::{error::Error, process};
use tracing::error;

use axum::{
    Json, Router,
    extract::DefaultBodyLimit,
    routing::{delete, get, post},
};

use crate::{
    constants::{MAX_SIZE_LIMIT_100MB, SERVE_ADDR},
    display_thread::{self, DisplayCommand},
    webserver::{
        delete_handler::delete_handler,
        display_handler::{handle_display_names, set_display_handler},
        image_handler::{handle_image, handle_image_names},
        upload_handler::upload_handler,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub tx: tokio::sync::mpsc::Sender<DisplayCommand>,
}

pub async fn webserver_main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = tokio::sync::mpsc::channel::<DisplayCommand>(10);
    std::thread::spawn(|| {
        if let Err(e) = display_thread::display_thread(rx) {
            error!("Critical error in display thread: {}", e.to_string());
            process::exit(1);
        }
    });
    let appstate = AppState { tx };

    let app = Router::new()
        .route("/api/images/{filename}", get(handle_image))
        .route("/api/images", get(handle_image_names))
        .route("/api/upload", post(upload_handler))
        .route("/api/delete/{filename}", delete(delete_handler))
        .route("/api/displays", get(handle_display_names))
        .route("/api/displays/{id}/{filename}", post(set_display_handler))
        .with_state(appstate)
        .layer(DefaultBodyLimit::max(MAX_SIZE_LIMIT_100MB))
        .fallback(static_handler::static_handler);

    let listener = tokio::net::TcpListener::bind(SERVE_ADDR).await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}
