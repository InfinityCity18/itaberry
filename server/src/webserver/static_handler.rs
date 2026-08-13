use axum::{
    http::{HeaderValue, StatusCode, header},
    response::IntoResponse,
};
use rust_embed::Embed;
use tracing::{error, instrument, trace};

#[derive(Embed)]
#[folder = "../front/dist/"]
struct Dist;

#[instrument]
pub async fn static_handler(uri: axum::http::Uri) -> impl IntoResponse {
    trace!(
        "Called static handler with uri: {uri}
    "
    );
    let mut path = uri.path().trim_start_matches('/');

    if path.is_empty() {
        path = "index.html";
    }

    match Dist::get(path) {
        Some(f) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime.as_ref()).unwrap(),
                )],
                f.data,
            )
                .into_response()
        }
        None => {
            error!("File: {path} not found in dist/");
            (
                StatusCode::NOT_FOUND,
                format!("File: {path} not found in dist/"),
            )
                .into_response()
        }
    }
}
