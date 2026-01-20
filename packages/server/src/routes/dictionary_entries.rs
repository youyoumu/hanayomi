use crate::{
    db::tables::DictionaryEntry,
    util::{
        response::{HandlerResult, RejectionResponse, success},
        state::AppState,
    },
};
use axum::extract::{Query, State};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct IndexQueryParams {
    #[validate(length(min = 1))]
    pub expression: String,
}

pub async fn search(
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
