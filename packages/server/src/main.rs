mod app;
mod cli;
mod routes;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::cli().await?;
    Ok(())
}

