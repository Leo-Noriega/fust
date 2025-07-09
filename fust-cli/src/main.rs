//! Fust CLI - A modern CLI application built with Rust

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod cli_adapter;
mod commands;
mod config;
mod error;

use cli_adapter::*;
use commands::*;
use config::*;

#[derive(Parser)]
#[command(
    name = "fust",
    about = "A modern CLI application built with Rust",
    version,
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new workspace
    Init {
        /// Workspace path
        path: Option<String>,
    },
    /// List all projects
    List,
    /// Add a new project
    Add {
        /// Project name
        name: String,
        /// Project path
        path: String,
        /// Project description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Remove a project
    Remove {
        /// Project name or ID
        project: String,
    },
    /// Open a project
    Open {
        /// Project name or ID
        project: String,
    },
    /// Show project information
    Show {
        /// Project name or ID
        project: String,
    },
    /// Search projects
    Search {
        /// Search query
        query: String,
    },
    /// Configure the application
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// List directories in a path
    Fud {
        /// Path to list directories from (defaults to current directory)
        path: Option<String>,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(if cli.verbose { Level::DEBUG } else { Level::INFO })
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Starting fust CLI");

    // Load configuration
    let config = load_config(cli.config.as_deref())?;

    // Execute command
    match cli.command {
        Commands::Init { path } => {
            let init_cmd = InitCommand::new(config);
            init_cmd.execute(path).await?;
        }
        Commands::List => {
            let list_cmd = ListCommand::new(config);
            list_cmd.execute().await?;
        }
        Commands::Add { name, path, description } => {
            let add_cmd = AddCommand::new(config);
            add_cmd.execute(name, path, description).await?;
        }
        Commands::Remove { project } => {
            let remove_cmd = RemoveCommand::new(config);
            remove_cmd.execute(project).await?;
        }
        Commands::Open { project } => {
            let open_cmd = OpenCommand::new(config);
            open_cmd.execute(project).await?;
        }
        Commands::Show { project } => {
            let show_cmd = ShowCommand::new(config);
            show_cmd.execute(project).await?;
        }
        Commands::Search { query } => {
            let search_cmd = SearchCommand::new(config);
            search_cmd.execute(query).await?;
        }
        Commands::Config { command } => {
            let mut config_cmd = ConfigCommand::new(config);
            match command {
                ConfigCommands::Show => config_cmd.show().await?,
                ConfigCommands::Set { key, value } => config_cmd.set(key, value).await?,
                ConfigCommands::Get { key } => config_cmd.get(key).await?,
            }
        }
        Commands::Fud { path } => {
            let fud_cmd = FudCommand::new()?;
            fud_cmd.execute_with_error_handling(path.as_deref())?;
        }
    }

    info!("fust CLI completed successfully");
    Ok(())
} 