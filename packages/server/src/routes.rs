use crate::util::state::AppState;
use axum::{Router, routing::get};
use tower_http::catch_panic::CatchPanicLayer;

mod definition_tags;
mod dictionaries;
mod dictionary_entries;
mod health;
mod index;
mod tokenize;

#[rustfmt::skip]
pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(index::root))
        .route("/health", get(health::status))
        .route("/dictionary_entries/search", get(dictionary_entries::search))
        .route("/definition_tags/search", get(definition_tags::search))
        .route("/dictionaries/{dictionary_id}", get(dictionaries::show))
        .route("/tokenize", get(tokenize::handle))
        .with_state(state)
        .layer(CatchPanicLayer::new())
}
