use crate::util::error::ErrorResponse;
use axum::{extract::Path, http::StatusCode};

pub async fn index(Path(expression): Path<String>) -> Result<(StatusCode, String), ErrorResponse> {
    // For now, we just return the captured kanji back in the response
    // let definition = db.query_dict(expression).await?;
    let response_message = format!("Querying for: {}", expression);

    Ok((StatusCode::OK, response_message))
}
