use axum::{Router, routing::get};
use tower_http::catch_panic::CatchPanicLayer;

mod health;
mod index;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/health", get(health::index))
        .layer(CatchPanicLayer::new())
}
