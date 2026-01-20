use crate::util::state::AppState;
use axum::{Router, routing::get};
use tower_http::catch_panic::CatchPanicLayer;

mod health;
mod index;
mod query;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/health", get(health::index))
        .route("/query/{expression}", get(query::expression))
        .with_state(state)
        .layer(CatchPanicLayer::new())
}
