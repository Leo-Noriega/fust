use crate::commands::{fud, fuf, init};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fust")]
#[command(version, about = "A terminal-based fuzzy finder and file manager")]
#[command(
    after_help = "QUICK SETUP:\n  1. fust init zsh >> ~/.zshrc  # Generate integration\n  2. source ~/.zshrc           # Reload shell\n  3. fud                       # Start using!"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Direct fuzzy directory navigation
    Fud,
    /// Direct fuzzy file navigation
    Fuf,
    /// Generate shell integration code
    Init {
        /// Shell type (bash, zsh, fish)
        shell: String,
    },
    /// Show detailed installation instructions
    Install,
}

impl Cli {
    pub async fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            Commands::Fud => fud::execute().await,
            Commands::Fuf => fuf::execute().await,
            Commands::Init { shell } => init::execute(&shell).await,
            Commands::Install => {
                init::print_installation_instructions();
                Ok(())
            }
        }
    }
}
