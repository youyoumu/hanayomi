use axum::{Router, routing::get};
use tower_http::catch_panic::CatchPanicLayer;

mod health;
mod index;
mod query;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/health", get(health::index))
        .route("/query/:expression", get(query::index))
        .layer(CatchPanicLayer::new())
}
