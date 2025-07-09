//! Domain models for file and directory metadata

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Metadata for a file
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileMetadata {
    /// File path
    pub path: PathBuf,
    /// File size in bytes
    pub size: u64,
    /// File permissions
    pub permissions: u32,
    /// Last modified time
    pub modified: SystemTime,
}

impl FileMetadata {
    /// Create a new file metadata
    pub fn new(path: PathBuf, size: u64, permissions: u32, modified: SystemTime) -> Self {
        Self {
            path,
            size,
            permissions,
            modified,
        }
    }

    /// Get file extension
    pub fn extension(&self) -> Option<&str> {
        self.path.extension()?.to_str()
    }

    /// Check if file is executable
    pub fn is_executable(&self) -> bool {
        self.permissions & 0o111 != 0
    }

    /// Check if file is readable
    pub fn is_readable(&self) -> bool {
        self.permissions & 0o444 != 0
    }

    /// Check if file is writable
    pub fn is_writable(&self) -> bool {
        self.permissions & 0o222 != 0
    }
}

/// Metadata for a directory
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DirectoryMetadata {
    /// Directory path
    pub path: PathBuf,
    /// Creation time
    pub created: SystemTime,
}

impl DirectoryMetadata {
    /// Create a new directory metadata
    pub fn new(path: PathBuf, created: SystemTime) -> Self {
        Self { path, created }
    }

    /// Get directory name
    pub fn name(&self) -> Option<&str> {
        self.path.file_name()?.to_str()
    }

    /// Get parent directory
    pub fn parent(&self) -> Option<&Path> {
        self.path.parent()
    }
} 