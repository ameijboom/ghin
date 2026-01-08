use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tokio::fs::create_dir_all;

use crate::{
    github::client::Client,
    store::{Database, models::NewConfig},
};

pub mod commands;
pub mod github;
pub mod package;
pub mod store;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(env = "XDG_CONFIG_HOME", default_value = "~/.config")]
    home: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(help = "Github repository in the form of <owner>/<repo>")]
        repo: String,
        #[arg(help = "Specific release tag to install")]
        #[arg(short, long)]
        tag: Option<String>,
        #[arg(help = "Alternative binary name")]
        #[arg(short, long)]
        name: Option<String>,
        #[arg(help = "Alternative installation directory")]
        #[arg(short, long)]
        directory: Option<String>,
    },
    #[command(alias = "rm")]
    Remove {
        #[arg(help = "Github repository in the form of <owner>/<repo>")]
        repo: String,
    },
    #[command(alias = "up")]
    Update {
        #[arg(help = "Github repository in the form of <owner>/<repo>")]
        repo: Option<String>,
    },
    Config {
        #[arg(
            long,
            help = "Sets the default installation directory, defaults to `/usr/local/bin`"
        )]
        install: Option<PathBuf>,
        #[arg(
            long,
            help = "Sets the default temp/download directory, defaults to `/tmp`"
        )]
        temp: Option<PathBuf>,
    },
}

async fn init(path: PathBuf) -> anyhow::Result<Database> {
    let mut needs_config = false;

    if !path.exists() {
        create_dir_all(&path).await?;
        needs_config = true;
    }

    let db = Database::new(path.join("ghin.db").to_str().unwrap())?;

    db.migrate().await?;

    if needs_config {
        db.set_config(NewConfig {
            installation_directory: None,
            temp_directory: None,
        })
        .await?;
    }

    Ok(db)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let db = init(args.home).await?;
    let client = Client::new();

    match args.command {
        Commands::Config { install, temp } => commands::config::run((install, temp), &db).await,
        Commands::Add {
            repo,
            tag,
            name,
            directory,
        } => commands::add::run((repo, tag, name, directory), &client, &db).await,
        _ => Ok(()),
    }
}
