//! CLI adapter for file system operations
//!
//! This module provides CLI adapters that connect the application layer
//! with the command-line interface, following hexagonal architecture principles.

use anyhow::Result;
use fust_adapter::filesystem_adapter::FileSystemAdapter;
use fust_application::list_directories::ListDirectories;
use fust_port::file_system::FsError;
use std::path::Path;
use tracing::{error, info};

/// CLI adapter for the fud subcommand
///
/// This adapter orchestrates the list directories functionality
/// by injecting the FileSystemAdapter into the ListDirectories use case.
#[derive(Debug)]
pub struct FudCommand {
    /// The list directories use case
    use_case: ListDirectories<FileSystemAdapter>,
}

impl FudCommand {
    /// Create a new FudCommand instance
    ///
    /// # Returns
    /// * `Result<Self, FsError>` - The command instance or error
    pub fn new() -> Result<Self, FsError> {
        // Create the filesystem adapter
        let adapter = FileSystemAdapter::new()?;

        // Create the use case with injected adapter
        let use_case = ListDirectories::new(adapter);

        Ok(Self { use_case })
    }

    /// Execute the fud command
    ///
    /// Lists directories in the specified path and prints their names
    /// to stdout, separated by newlines.
    ///
    /// # Arguments
    /// * `path` - Optional path to list directories from (defaults to current directory)
    ///
    /// # Returns
    /// * `Result<(), FsError>` - Success or error
    pub fn execute(&self, path: Option<&str>) -> Result<(), FsError> {
        // Determine the path to use
        let target_path = match path {
            Some(p) => Path::new(p),
            None => Path::new("."), // Current directory
        };

        info!("Executing fud command for path: {:?}", target_path);

        // Execute the use case
        let directories = self.use_case.execute(target_path)?;

        // Print directory names to stdout, separated by newlines
        for directory in directories {
            println!("{}", directory.name().unwrap_or("unknown"));
        }

        info!("fud command completed successfully");
        Ok(())
    }

    /// Execute the fud command with error handling
    ///
    /// This method provides proper error handling and user-friendly
    /// error messages for CLI usage.
    ///
    /// # Arguments
    /// * `path` - Optional path to list directories from
    ///
    /// # Returns
    /// * `Result<(), anyhow::Error>` - Success or error
    pub fn execute_with_error_handling(&self, path: Option<&str>) -> Result<(), anyhow::Error> {
        match self.execute(path) {
            Ok(()) => Ok(()),
            Err(FsError::NotFound(path_str)) => {
                error!("Path not found: {}", path_str);
                anyhow::bail!("Path not found: {}", path_str);
            }
            Err(FsError::PermissionDenied(reason)) => {
                error!("Permission denied: {}", reason);
                anyhow::bail!("Permission denied: {}", reason);
            }
            Err(FsError::NotImplemented(feature)) => {
                error!("Feature not implemented: {}", feature);
                anyhow::bail!("Feature not implemented: {}", feature);
            }
            Err(FsError::Io(error_msg)) => {
                error!("IO error: {}", error_msg);
                anyhow::bail!("IO error: {}", error_msg);
            }
            Err(FsError::InvalidPath(path_str)) => {
                error!("Invalid path: {}", path_str);
                anyhow::bail!("Invalid path: {}", path_str);
            }
        }
    }
}
