mod cli;
mod db;
mod routes;
mod schemas;
mod server;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::cli().await?;
    Ok(())
}
