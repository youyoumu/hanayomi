mod app;
mod error;
mod routes;

#[tokio::main]
async fn main() {
    let _ = app::app().await;
}
