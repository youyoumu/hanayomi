use crate::db::Db;
use crate::server::serve;
use crate::util::config::Config;
use crate::util::dict::Dict;
use clap::{Parser, Subcommand};
use serde_json::json;

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
        dictionary: String,
    },

    #[command(about = "Check the dictionary")]
    Check {
        #[arg(long)]
        workdir: Option<String>,
    },

    #[command(about = "Query the dictionary")]
    Query {
        #[arg(long)]
        workdir: Option<String>,
        #[arg(long)]
        expression: String,
    },
}

pub async fn cli() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let host = "127.0.0.1".to_string();
    let port = 45636;

    match cli.command {
        Commands::Serve {
            port,
            host,
            workdir,
        } => {
            let config = Config::new(workdir, host, port)?;
            serve(&config).await?
        }
        Commands::Dict { action } => match action {
            DictCommands::Parse {
                workdir,
                dictionary,
            } => {
                let config = Config::new(workdir, host, port)?;
                let db = Db::new(&config).await?;
                let dict = Dict::new(&config);
                dict.parse_dict(dictionary, &db).await?;
            }
            DictCommands::Check { workdir } => {
                println!("Checking dictionary...");
                let _config = Config::new(workdir, host, port)?;
            }
            DictCommands::Query {
                workdir,
                expression,
            } => {
                let config = Config::new(workdir, host, port)?;
                let db = Db::new(&config).await?;
                let definition = db.query_dict(expression).await?;
                println!("{}", json!(definition));
            }
        },
    };

    Ok(())
}
