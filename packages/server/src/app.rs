use crate::routes::create_routes;
use anyhow::Result;

pub async fn app(host: String, port: u16) -> Result<()> {
    let app = create_routes();
    let address = format!("{}:{}", host, port);

    println!("starting server at {}", address);

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
