use crate::server::serve;
use crate::util::dict::parse_dict;
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
    #[command(about = "Parse the dictionary")]
    Parse {
        #[arg(long)]
        workdir: Option<String>,

        #[arg(long)]
        dict: String,
    },

    #[command(about = "Check the dictionary")]
    Check,
}

pub async fn cli() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { port, host } => serve(host, port).await?,
        Commands::Dict { action } => match action {
            DictCommands::Parse { workdir, dict } => {
                println!("Parsing dictionary...");
                parse_dict(workdir, dict)?;
            }
            DictCommands::Check => {
                println!("Checking dictionary...");
            }
        },
    };

    Ok(())
}
