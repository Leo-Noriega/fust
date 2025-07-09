//! Directory service for file system operations
//! 
//! This module provides domain services for directory operations
//! using real file system operations with tokio.

use crate::fs_errors::FsError;
use std::path::Path;
use tokio::fs;
use tracing::{error, info};

/// Directory service for file system operations
/// 
/// This service provides domain-level operations for directories
/// using real file system operations with proper error handling.
#[derive(Debug, Clone)]
pub struct DirectoryService;

impl DirectoryService {
    /// Create a new DirectoryService instance
    /// 
    /// # Returns
    /// * `Self` - The service instance
    pub fn new() -> Self {
        Self
    }

    /// Create a directory and all its parent directories
    /// 
    /// This method creates the specified directory and all necessary
    /// parent directories. If the directory already exists, it succeeds
    /// (idempotent operation).
    /// 
    /// # Arguments
    /// * `path` - The path to create
    /// 
    /// # Returns
    /// * `Result<(), FsError>` - Success or error
    pub async fn create(&self, path: &Path) -> Result<(), FsError> {
        info!("Creating directory: {:?}", path);
        
        // Validate path
        if let Some(invalid_char) = path.to_string_lossy().chars().find(|c| *c == '\0') {
            return Err(FsError::InvalidPath(format!(
                "Path contains invalid character: {:?}",
                invalid_char
            )));
        }
        
        match fs::create_dir_all(path).await {
            Ok(()) => {
                info!("Successfully created directory: {:?}", path);
                Ok(())
            }
            Err(e) => {
                error!("Failed to create directory {:?}: {}", path, e);
                match e.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        Err(FsError::PermissionDenied(format!(
                            "Permission denied creating directory: {}",
                            path.display()
                        )))
                    }
                    std::io::ErrorKind::NotFound => {
                        Err(FsError::NotFound(format!(
                            "Parent directory not found: {}",
                            path.display()
                        )))
                    }
                    std::io::ErrorKind::InvalidInput => {
                        Err(FsError::InvalidPath(format!(
                            "Invalid path: {}",
                            path.display()
                        )))
                    }
                    _ => {
                        Err(FsError::Io(format!(
                            "IO error creating directory {}: {}",
                            path.display(),
                            e
                        )))
                    }
                }
            }
        }
    }

    /// Rename a directory
    /// 
    /// This method renames a directory from old_path to new_path.
    /// If the new path already exists, the operation will fail.
    /// 
    /// # Arguments
    /// * `old_path` - The current path of the directory
    /// * `new_path` - The new path for the directory
    /// 
    /// # Returns
    /// * `Result<(), FsError>` - Success or error
    pub async fn rename(&self, old_path: &Path, new_path: &Path) -> Result<(), FsError> {
        info!("Renaming directory from {:?} to {:?}", old_path, new_path);
        
        // Validate paths
        for path in [old_path, new_path] {
            if let Some(invalid_char) = path.to_string_lossy().chars().find(|c| *c == '\0') {
                return Err(FsError::InvalidPath(format!(
                    "Path contains invalid character: {:?}",
                    invalid_char
                )));
            }
        }
        
        // Check if old path exists
        if !old_path.exists() {
            return Err(FsError::NotFound(format!(
                "Directory not found: {}",
                old_path.display()
            )));
        }
        
        // Check if new path already exists
        if new_path.exists() {
            return Err(FsError::Io(format!(
                "Destination already exists: {}",
                new_path.display()
            )));
        }
        
        match fs::rename(old_path, new_path).await {
            Ok(()) => {
                info!("Successfully renamed directory from {:?} to {:?}", old_path, new_path);
                Ok(())
            }
            Err(e) => {
                error!("Failed to rename directory from {:?} to {:?}: {}", old_path, new_path, e);
                match e.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        Err(FsError::PermissionDenied(format!(
                            "Permission denied renaming directory: {}",
                            old_path.display()
                        )))
                    }
                    std::io::ErrorKind::NotFound => {
                        Err(FsError::NotFound(format!(
                            "Directory not found: {}",
                            old_path.display()
                        )))
                    }
                    std::io::ErrorKind::AlreadyExists => {
                        Err(FsError::Io(format!(
                            "Destination already exists: {}",
                            new_path.display()
                        )))
                    }
                    _ => {
                        Err(FsError::Io(format!(
                            "IO error renaming directory {} to {}: {}",
                            old_path.display(),
                            new_path.display(),
                            e
                        )))
                    }
                }
            }
        }
    }

    /// Delete a directory and all its contents
    /// 
    /// This method removes a directory and all its contents recursively.
    /// If the directory doesn't exist, the operation will fail.
    /// 
    /// # Arguments
    /// * `path` - The path to delete
    /// 
    /// # Returns
    /// * `Result<(), FsError>` - Success or error
    pub async fn delete(&self, path: &Path) -> Result<(), FsError> {
        info!("Deleting directory: {:?}", path);
        
        // Validate path
        if let Some(invalid_char) = path.to_string_lossy().chars().find(|c| *c == '\0') {
            return Err(FsError::InvalidPath(format!(
                "Path contains invalid character: {:?}",
                invalid_char
            )));
        }
        
        // Check if path exists
        if !path.exists() {
            return Err(FsError::NotFound(format!(
                "Directory not found: {}",
                path.display()
            )));
        }
        
        match fs::remove_dir_all(path).await {
            Ok(()) => {
                info!("Successfully deleted directory: {:?}", path);
                Ok(())
            }
            Err(e) => {
                error!("Failed to delete directory {:?}: {}", path, e);
                match e.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        Err(FsError::PermissionDenied(format!(
                            "Permission denied deleting directory: {}",
                            path.display()
                        )))
                    }
                    std::io::ErrorKind::NotFound => {
                        Err(FsError::NotFound(format!(
                            "Directory not found: {}",
                            path.display()
                        )))
                    }
                    _ => {
                        Err(FsError::Io(format!(
                            "IO error deleting directory {}: {}",
                            path.display(),
                            e
                        )))
                    }
                }
            }
        }
    }
} 