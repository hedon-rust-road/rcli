use std::{path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    dir: PathBuf,
}

pub async fn process_http_serve(dir: PathBuf, port: u16) -> anyhow::Result<()> {
    info!("Serving {:?} on 0.0.0.0:{port}", dir);
    let state = HttpServeState { dir };

    let app = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let full_path = state.dir.join(path);
    info!("Reading file: {:?}", full_path);
    if !full_path.exists() {
        return (
            StatusCode::NOT_FOUND,
            format!("File {} not fould", full_path.display()),
        );
    }
    match tokio::fs::read_to_string(full_path).await {
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        Ok(content) => (StatusCode::OK, content),
    }
}