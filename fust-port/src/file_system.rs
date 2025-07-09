//! File system port interfaces
//! 
//! This module defines the interfaces for file system operations
//! following the hexagonal architecture pattern.

use fust_domain::metadata::{DirectoryMetadata, FileMetadata};
use std::path::Path;
use thiserror::Error;

/// Error types for file system operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum FsError {
    #[error("IO error: {0}")]
    Io(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Path not found: {0}")]
    NotFound(String),
    
    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

/// Error types for fuzzy finder operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum FinderError {
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
    
    #[error("No matches found")]
    NoMatches,
    
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Repository trait for file operations
pub trait FileRepository: Send + Sync {
    /// List files in a directory
    /// 
    /// # Arguments
    /// * `dir` - The directory path to list files from
    /// 
    /// # Returns
    /// * `Result<Vec<FileMetadata>, FsError>` - List of file metadata or error
    fn list_files(&self, dir: &Path) -> Result<Vec<FileMetadata>, FsError>;
}

/// Repository trait for directory operations
pub trait DirectoryRepository: Send + Sync {
    /// List directories in a directory
    /// 
    /// # Arguments
    /// * `dir` - The directory path to list subdirectories from
    /// 
    /// # Returns
    /// * `Result<Vec<DirectoryMetadata>, FsError>` - List of directory metadata or error
    fn list_directories(&self, dir: &Path) -> Result<Vec<DirectoryMetadata>, FsError>;
}

/// Generic fuzzy finder trait
/// 
/// This trait provides fuzzy search functionality for any type T.
/// It returns indices of matching items in the provided slice.
pub trait FuzzyFinder<T>: Send + Sync {
    /// Find items that match the fuzzy search criteria
    /// 
    /// # Arguments
    /// * `items` - Slice of items to search through
    /// 
    /// # Returns
    /// * `Result<Vec<usize>, FinderError>` - Indices of matching items or error
    fn find(&self, items: &[T]) -> Result<Vec<usize>, FinderError>;
} 