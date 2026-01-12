mod app;
mod routes;
mod util;

#[tokio::main]
async fn main() {
    let _ = app::app().await;
}
