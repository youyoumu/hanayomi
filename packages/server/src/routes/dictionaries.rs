use crate::{
    db::tables::Dictionary,
    util::{
        response::{HandlerResult, RejectionResponse, fail, success},
        state::AppState,
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::WithRejection;

pub async fn index(State(state): State<AppState>) -> HandlerResult<Vec<Dictionary>> {
    let dictionaries = state.db.query_dictionaries().await?;
    success(dictionaries)
}

pub async fn show(
    State(state): State<AppState>,
    WithRejection(Path(dictionary_id), _): WithRejection<Path<i32>, RejectionResponse>,
) -> HandlerResult<Dictionary> {
    let dictionary = state.db.query_dictionary(dictionary_id).await?;
    match dictionary {
        Some(dictionary) => success(dictionary),
        None => fail("Dictionary not found".to_string(), StatusCode::NOT_FOUND),
    }
}

pub async fn destroy(
    State(state): State<AppState>,
    WithRejection(Path(dictionary_id), _): WithRejection<Path<i32>, RejectionResponse>,
) -> HandlerResult<Dictionary> {
    let dictionary = state.db.query_delete_dictionary(dictionary_id).await?;
    match dictionary {
        Some(dictionary) => success(dictionary),
        None => fail("Dictionary not found".to_string(), StatusCode::NOT_FOUND),
    }
}
