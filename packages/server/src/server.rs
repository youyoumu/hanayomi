use crate::{
    db::Db,
    routes::create_routes,
    util::{config::Config, lexer, state::AppState},
};
use anyhow::Context;
use std::sync::Arc;

pub async fn serve(config: Arc<Config>) -> anyhow::Result<()> {
    let db = Db::new(config.clone()).await?;
    let db = Arc::new(db);
    let lexer = lexer::Lexer::new()?;
    let lexer = Arc::new(lexer);
    let state = AppState {
        db: db.clone(),
        lexer: lexer.clone(),
    };

    let app = create_routes(state);
    let address = format!("{}:{}", config.server.host, config.server.port);

    println!("Starting server at {}", address);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .context("Failed bind TcpListener")?;
    axum::serve(listener, app)
        .await
        .context("Failed to serve HTTP routes")?;

    Ok(())
}
