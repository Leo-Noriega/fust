mod cli;
mod commands;
mod config;
mod error;
mod file_ops;
mod tui;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    cli.execute().await
}
