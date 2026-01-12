use axum::http::StatusCode;

use crate::error::ErrorResponse;

pub async fn index() -> Result<(StatusCode, &'static str), ErrorResponse> {
    Ok((StatusCode::OK, "Ok"))
}
