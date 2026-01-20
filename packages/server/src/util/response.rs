use axum::{Json, extract::rejection::QueryRejection, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

pub type HandlerResult<T> = Result<(StatusCode, Json<Response<T>>), ErrorResponse>;

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Fail,
    Error,
}

pub fn success<T>(data: T) -> HandlerResult<T> {
    Ok((
        StatusCode::OK,
        Json(Response {
            status: ResponseStatus::Success,
            data: Some(data),
            message: None,
        }),
    ))
}

pub fn fail<T>(message: String, status_code: StatusCode) -> HandlerResult<T> {
    Ok((
        status_code,
        Json(Response {
            status: ResponseStatus::Fail,
            data: None,
            message: Some(message),
        }),
    ))
}

pub fn error<T>(message: String) -> HandlerResult<T> {
    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(Response {
            status: ResponseStatus::Error,
            data: None,
            message: Some(message),
        }),
    ))
}

pub struct ErrorResponse {
    pub error: anyhow::Error,
    pub status_code: StatusCode,
}

impl From<anyhow::Error> for ErrorResponse {
    fn from(value: anyhow::Error) -> Self {
        Self {
            error: value,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<ValidationErrors> for ErrorResponse {
    fn from(value: ValidationErrors) -> Self {
        let err = anyhow::anyhow!(value.to_string());
        Self {
            error: err,
            status_code: StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        if self.status_code == StatusCode::INTERNAL_SERVER_ERROR {
            error::<()>(self.error.to_string()).into_response()
        } else {
            fail::<()>(self.error.to_string(), self.status_code).into_response()
        }
    }
}

pub struct RejectionResponse {
    message: String,
}

impl From<QueryRejection> for RejectionResponse {
    fn from(value: QueryRejection) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl IntoResponse for RejectionResponse {
    fn into_response(self) -> axum::response::Response {
        fail::<()>(self.message, StatusCode::BAD_REQUEST).into_response()
    }
}
