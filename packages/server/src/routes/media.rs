use crate::util::{response::ErrorResponse, state::AppState};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, Response, StatusCode},
};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[axum::debug_handler]
pub async fn serve(
    State(state): State<AppState>,
    Path((dictionary_id, relative_path)): Path<(i32, String)>,
) -> Result<Response<Body>, ErrorResponse> {
    let dict_dir = state.config.dir.dict.join(dictionary_id.to_string());
    let file_path = dict_dir.join(relative_path);

    let mut file = File::open(&file_path).await.map_err(|_| ErrorResponse {
        error: anyhow::anyhow!("File not found"),
        status_code: StatusCode::NOT_FOUND,
    })?;

    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .await
        .map_err(|e| ErrorResponse {
            error: anyhow::anyhow!("Failed to read file: {}", e),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CACHE_CONTROL, "public, max-age=60")
        .body(Body::from(contents))
        .map_err(|e| ErrorResponse {
            error: anyhow::anyhow!("Failed to create response: {}", e),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(response)
}
