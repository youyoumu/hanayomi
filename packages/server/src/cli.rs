use crate::db::init_db;
use crate::util::dict::parse_dict;
use crate::{server::serve, util::config::init_config};
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

        #[arg(long)]
        workdir: Option<String>,
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
    Check {
        #[arg(long)]
        workdir: Option<String>,
    },
}

pub async fn cli() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve {
            port,
            host,
            workdir,
        } => {
            init_config(workdir)?;
            serve(host, port).await?
        }
        Commands::Dict { action } => match action {
            DictCommands::Parse { workdir, dict } => {
                println!("Parsing dictionary...");
                init_config(workdir.clone())?;
                init_db().await?;
                parse_dict(dict)?;
            }
            DictCommands::Check { workdir } => {
                println!("Checking dictionary...");
                init_config(workdir)?;
            }
        },
    };

    Ok(())
}
