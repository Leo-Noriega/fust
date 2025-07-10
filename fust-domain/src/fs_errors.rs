//! File system error types
//!
//! This module defines error types for file system operations
//! in the domain layer to avoid circular dependencies.

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

    #[error("Feature not implemented: {0}")]
    NotImplemented(String),
}
