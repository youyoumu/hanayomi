use anyhow::Context;

use crate::routes::create_routes;

pub async fn serve(host: String, port: u16) -> anyhow::Result<()> {
    let app = create_routes();
    let address = format!("{}:{}", host, port);

    println!("Starting server at {}", address);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .context("Failed bind TcpListener")?;
    axum::serve(listener, app)
        .await
        .context("Failed to serve HTTP routes")?;

    Ok(())
}
