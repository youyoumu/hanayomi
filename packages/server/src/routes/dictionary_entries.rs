use crate::{
    db::tables::DictionaryEntry,
    util::{
        response::{HandlerResult, fail, success},
        state::AppState,
    },
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use std::collections::HashMap;

pub async fn index(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> HandlerResult<Vec<DictionaryEntry>> {
    let expression = params.get("expression");

    match expression {
        Some(expression) => {
            let definition = state
                .db
                .query_dictionary_entry_by(expression.clone())
                .await?;
            success(definition)
        }
        None => fail(
            "Missing query params: expression".to_string(),
            StatusCode::BAD_REQUEST,
        ),
    }
}
