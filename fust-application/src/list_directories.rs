//! List directories use case
//! 
//! This module contains the use case for listing directories in the file system.
//! It follows the hexagonal architecture pattern by depending on abstractions (ports)
//! rather than concrete implementations.

use fust_domain::metadata::DirectoryMetadata;
use fust_port::file_system::{DirectoryRepository, FsError};
use std::path::Path;

/// Use case for listing directories in a given path
/// 
/// This use case orchestrates the listing of directories by delegating
/// to a DirectoryRepository implementation. It follows the dependency
/// inversion principle by depending on the DirectoryRepository trait
/// rather than concrete implementations.
#[derive(Debug, Clone)]
pub struct ListDirectories<U>
where
    U: DirectoryRepository,
{
    /// The directory repository implementation
    repo: U,
}

impl<U> ListDirectories<U>
where
    U: DirectoryRepository,
{
    /// Create a new ListDirectories use case
    /// 
    /// # Arguments
    /// * `repo` - The directory repository implementation
    /// 
    /// # Returns
    /// * `Self` - The use case instance
    pub fn new(repo: U) -> Self {
        Self { repo }
    }

    /// Execute the list directories use case
    /// 
    /// This method lists all directories in the specified path by delegating
    /// to the injected DirectoryRepository implementation.
    /// 
    /// # Arguments
    /// * `path` - The path to list directories from
    /// 
    /// # Returns
    /// * `Result<Vec<DirectoryMetadata>, FsError>` - List of directory metadata or error
    pub fn execute(&self, path: &Path) -> Result<Vec<DirectoryMetadata>, FsError> {
        // Delegate to the repository implementation
        self.repo.list_directories(path)
    }
} 