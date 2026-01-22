use crate::server::serve;
use crate::util::config::Config;
use crate::util::dict::Dict;
use crate::{db::Db, util::lexer::Lexer};
use clap::{Parser, Subcommand};
use serde_json::json;
use std::sync::Arc;

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

    #[command(about = "Manage the Lexer")]
    Lexer {
        #[command(subcommand)]
        action: LexerCommands,
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

#[derive(Subcommand, Debug)]
enum LexerCommands {
    #[command(about = "Tokenize s sentence")]
    Tokenize {
        #[arg(long)]
        sentence: String,
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
            let config = Arc::new(config);
            serve(config.clone()).await?
        }
        Commands::Dict { action } => match action {
            DictCommands::Parse {
                workdir,
                dictionary,
            } => {
                let config = Config::new(workdir, host, port)?;
                let config = Arc::new(config);
                let db = Db::new(config.clone()).await?;
                let dict = Dict::new(config.clone());
                dict.parse_dict(dictionary, db).await?;
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
                let config = Arc::new(config);
                let db = Db::new(config.clone()).await?;
                let definition = db.query_dictionary_entry_by(expression).await?;
                println!("{}", json!(definition));
            }
        },
        Commands::Lexer { action } => match action {
            LexerCommands::Tokenize { sentence } => {
                let lexer = Lexer::new()?;
                let tokens = lexer.tokenize(sentence);
                let json = serde_json::to_string(&tokens)?;
                println!("{}", json);
            }
        },
    };

    Ok(())
}
