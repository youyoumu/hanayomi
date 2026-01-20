use crate::{
    db::tables::DictionaryEntry,
    util::{error::ErrorResponse, state::AppState},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn expression(
    State(state): State<AppState>,
    Path(expression): Path<String>,
) -> Result<(StatusCode, Json<Vec<DictionaryEntry>>), ErrorResponse> {
    let definition = state.db.query_dict(expression.clone()).await?;

    Ok((StatusCode::OK, Json(definition)))
}
