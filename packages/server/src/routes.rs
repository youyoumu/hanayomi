use crate::util::state::AppState;
use axum::{Router, routing::get};
use tower_http::catch_panic::CatchPanicLayer;

mod definition_tags;
mod dictionaries;
mod dictionary_entries;
mod health;
mod index;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/health", get(health::index))
        .route("/dictionary_entries", get(dictionary_entries::index))
        .route("/definition_tags", get(definition_tags::index))
        .route("/dictionaries/{dictionary_id}", get(dictionaries::index))
        .with_state(state)
        .layer(CatchPanicLayer::new())
}
