use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::util::error::ErrorResponse;

pub type HandlerResult<T> = Result<(StatusCode, Json<Response<T>>), ErrorResponse>;

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    status: ResponseStatus,
    data: Option<T>,
    message: Option<String>,
}

#[derive(Serialize, Deserialize)]
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
