mod cli;
mod routes;
mod server;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::cli().await?;
    Ok(())
}
