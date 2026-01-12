use axum::http::StatusCode;

use crate::error::ResponseError;

pub async fn index() -> Result<(StatusCode, &'static str), ResponseError> {
    Ok((StatusCode::OK, "Hello, world!"))
}

