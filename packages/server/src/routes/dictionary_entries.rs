use crate::{
    db::tables::DictionaryEntry,
    util::{
        response::{ErrorResponse, HandlerResult, Response, fail, success},
        state::AppState,
    },
};
use axum::{
    Json,
    extract::{Query, State, rejection::QueryRejection},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use std::collections::HashMap;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Deserialize, Validate)]
pub struct IndexParams {
    #[validate(length(min = 1))]
    pub expression: String,
}

pub struct AppRejection {
    message: String,
}

impl From<QueryRejection> for AppRejection {
    fn from(value: QueryRejection) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl IntoResponse for AppRejection {
    fn into_response(self) -> axum::response::Response {
        fail::<()>(self.message, StatusCode::BAD_REQUEST).into_response()
    }
}

pub async fn index(
    State(state): State<AppState>,
    WithRejection(Query(params), _): WithRejection<Query<IndexParams>, AppRejection>,
) -> HandlerResult<Vec<DictionaryEntry>> {
    params.validate()?;
    let expression = params.expression;

    let definition = state
        .db
        .query_dictionary_entry_by(expression.clone())
        .await?;
    success(definition)
}
