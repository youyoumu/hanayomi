use std::sync::Arc;

use anyhow::Context;

use crate::{db::Db, routes::create_routes, util::config::Config};

pub async fn serve(config: Arc<Config>) -> anyhow::Result<()> {
    let app = create_routes();
    let address = format!("{}:{}", config.server.host, config.server.port);
    let db = Db::new(config).await?;
    let shared_state = std::sync::Arc::new(db);

    println!("Starting server at {}", address);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .context("Failed bind TcpListener")?;
    axum::serve(listener, app)
        .await
        .context("Failed to serve HTTP routes")?;

    Ok(())
}
