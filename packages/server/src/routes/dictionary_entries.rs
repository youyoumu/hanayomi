use crate::{
    db::tables::DictionaryEntry,
    util::{error::ErrorResponse, state::AppState},
};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct IndexResponse {
    status: String,
    data: Option<Vec<DictionaryEntry>>,
    message: Option<String>,
}

pub async fn index(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<(StatusCode, Json<IndexResponse>), ErrorResponse> {
    let expression = params.get("expression");

    match expression {
        Some(expression) => {
            let definition = state
                .db
                .query_dictionary_entry_by(expression.clone())
                .await?;
            let response = IndexResponse {
                status: "success".to_string(),
                data: Some(definition),
                message: None,
            };
            Ok((StatusCode::OK, Json(response)))
        }
        None => {
            let response = IndexResponse {
                status: "fail".to_string(),
                data: None,
                message: Some("Missing query params: expression".to_string()),
            };
            Ok((StatusCode::BAD_REQUEST, Json(response)))
        }
    }
}
