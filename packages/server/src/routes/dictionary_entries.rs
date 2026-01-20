use crate::{
    db::tables::DictionaryEntry,
    util::{
        response::{ErrorResponse, HandlerResult, RejectionResponse, Response, fail, success},
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
pub struct IndexQueryParams {
    #[validate(length(min = 1))]
    pub expression: String,
}

pub async fn index(
    State(state): State<AppState>,
    WithRejection(Query(params), _): WithRejection<Query<IndexQueryParams>, RejectionResponse>,
) -> HandlerResult<Vec<DictionaryEntry>> {
    params.validate()?;
    let expression = params.expression;

    let definition = state
        .db
        .query_dictionary_entry_by(expression.clone())
        .await?;
    success(definition)
}
