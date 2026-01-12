use crate::util::error::ErrorResponse;
use axum::http::StatusCode;

pub async fn index() -> Result<(StatusCode, &'static str), ErrorResponse> {
    Ok((StatusCode::OK, "Ok"))
}
