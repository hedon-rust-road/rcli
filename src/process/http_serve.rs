use std::{
    fs::{self},
    io::Write,
    path::PathBuf,
    sync::Arc,
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    dir: PathBuf,
}

pub async fn process_http_serve(dir: PathBuf, port: u16) -> anyhow::Result<()> {
    info!("Serving {:?} on 0.0.0.0:{port}", dir);

    // Generate index.html for all sub directories.
    generate_indexes(dir.clone(), true)?;

    // Define axum state and service.
    let state = HttpServeState { dir: dir.clone() };
    let dir_service = ServeDir::new(dir)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_zstd()
        .precompressed_deflate();

    // Define router application.
    let app = Router::new()
        .nest_service("/tower", dir_service)
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    // Listen specified port.
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    // Start http server
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

fn generate_indexes(dir: PathBuf, first: bool) -> anyhow::Result<()> {
    let index_file = dir.join("index.html");
    if index_file.exists() {
        fs::remove_file(index_file.as_path())?;
    }
    let mut f = fs::File::create(index_file.as_path())?;
    let mut html = String::new();

    html.push_str(
        format!(
            "<!DOCTYPE html>\n<html>\n<head>\n<title>{}</title>\n</head>\n<body>\n",
            dir.display()
        )
        .as_str(),
    );
    html.push_str(format!("<h1>{} File List</h1>\n", dir.display()).as_str());

    // Add `..` for returning back pre directory.
    if !first {
        html.push_str("<li><a href=\"../\">../</a></li>\n");
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;

        // Ignore index.html itself.
        if entry
            .file_name()
            .into_string()
            .unwrap()
            .contains("index.html")
        {
            continue;
        }

        let path = entry.path();
        let filename = entry.file_name().into_string().unwrap();
        if path.is_dir() {
            html.push_str(&format!(
                "<li><a href=\"./{}/\">{}/</a></li>\n",
                filename, filename,
            ));
            generate_indexes(path, false)?;
        } else {
            // NOTE: tower-http supports viewing only files with type suffixes.
            if !filename.contains('.') || filename.starts_with('.') {
                continue;
            }
            html.push_str(&format!(
                "<li><a href=\"./{}\">{}</a></li>\n",
                filename, filename,
            ));
        }
    }
    html.push_str("</ul>\n");
    html.push_str("</body>\n</html>");
    f.write_all(html.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::PathBuf, sync::Arc};

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            dir: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
