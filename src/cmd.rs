use clap::{Parser, Subcommand};

use crate::config;

#[derive(Parser, Debug)]
#[command(
    name = "pinbox",
    about = "CLI tool for bookmarks stored in git",
    version = "0.1"
)]
pub struct Cmd {
    #[command(subcommand)]
    command: Commands,
}

impl Cmd {
    pub fn run() {
        let cli = Cmd::parse();

        match cli.command {
            Commands::Config { key, value } => {
                let _ =
                    config::process_config_cmd(key, value).inspect_err(|e| {
                        println!("failed to process cmd config: {}", e)
                    });
            }
            Commands::Pin { category, content } => {}
            Commands::Add { command } => match command {
                AddCommand::Category { name, alias } => {}
            },
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    Config {
        key: String,
        value: String,
    },
    Pin {
        category: String,
        content: Content,
    },
    Add {
        #[command(subcommand)]
        command: AddCommand,
    },
}

#[derive(Subcommand, Debug)]
enum AddCommand {
    Category {
        name: String,
        #[arg(short, long)]
        alias: Option<String>,
    },
}

#[derive(Clone, Debug)]
enum Content {
    Title(String),
    Url(url::Url),
}

impl std::str::FromStr for Content {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match url::Url::parse(s) {
            Ok(url) => {
                if url.scheme() == "https" || url.scheme() == "http" {
                    Ok(Self::Url(url))
                } else {
                    Ok(Self::Title(s.to_string()))
                }
            }
            Err(_) => Ok(Self::Title(s.to_string())),
        }
    }
}
