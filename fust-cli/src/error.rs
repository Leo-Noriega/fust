//! CLI error handling

use thiserror::Error;

/// CLI-specific error types
#[derive(Error, Debug)]
pub enum CliError {
    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Command execution failed: {message}")]
    Command { message: String },

    #[error("Invalid input: {message}")]
    InvalidInput { message: String },

    #[error("Project not found: {name}")]
    ProjectNotFound { name: String },

    #[error("Workspace not initialized")]
    WorkspaceNotInitialized,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl CliError {
    /// Create a new configuration error
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a new command error
    pub fn command(message: impl Into<String>) -> Self {
        Self::Command {
            message: message.into(),
        }
    }

    /// Create a new invalid input error
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    /// Create a new project not found error
    pub fn project_not_found(name: impl Into<String>) -> Self {
        Self::ProjectNotFound {
            name: name.into(),
        }
    }
} 