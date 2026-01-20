use axum::{http::StatusCode, response::IntoResponse};
use std::error::Error;
use validator::ValidationError;

pub struct ErrorResponse(pub anyhow::Error);

impl From<anyhow::Error> for ErrorResponse {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl From<serde_json::Error> for ErrorResponse {
    fn from(value: serde_json::Error) -> Self {
        Self(value.into())
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
