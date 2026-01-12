use crate::routes::create_routes;
use anyhow::Result;

pub async fn app() -> Result<()> {
    let app = create_routes();
    let address = "127.0.0.1:3000";

    println!("starting server at {}", address);

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
