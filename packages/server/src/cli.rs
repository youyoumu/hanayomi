use crate::app::app;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "hanayomi")]
#[command(version, about = "(TODO) Some dictionary tools", long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Start the server")]
    Serve {
        #[arg(long, default_value = "45636")]
        port: u16,

        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },

    #[command(about = "Manage the dictionary")]
    Dict {
        #[command(subcommand)]
        action: DictCommands,
    },
}

#[derive(Subcommand, Debug)]
enum DictCommands {
    #[command(about = "Check the dictionary")]
    Parse,

    #[command(about = "Check the dictionary")]
    Check,
}

pub async fn cli() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { port, host } => app(host, port).await?,
        Commands::Dict { action } => match action {
            DictCommands::Parse => {
                println!("Parsing dictionary...");
            }
            DictCommands::Check => {
                println!("Checking dictionary...");
            }
        },
    };

    Ok(())
}
