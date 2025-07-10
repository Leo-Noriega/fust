//! Value objects for the domain

use serde::{Deserialize, Serialize};
use std::fmt;

/// A file path value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FilePath(pub String);

impl FilePath {
    /// Create a new file path
    pub fn new(path: impl Into<String>) -> Result<Self, String> {
        let path = path.into();
        if path.is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        Ok(Self(path))
    }

    /// Get the path as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the file name
    pub fn file_name(&self) -> Option<&str> {
        std::path::Path::new(&self.0).file_name()?.to_str()
    }

    /// Get the parent directory
    pub fn parent(&self) -> Option<&str> {
        std::path::Path::new(&self.0).parent()?.to_str()
    }

    /// Check if the path exists
    pub fn exists(&self) -> bool {
        std::path::Path::new(&self.0).exists()
    }

    /// Check if it's a file
    pub fn is_file(&self) -> bool {
        std::path::Path::new(&self.0).is_file()
    }

    /// Check if it's a directory
    pub fn is_dir(&self) -> bool {
        std::path::Path::new(&self.0).is_dir()
    }
}

impl fmt::Display for FilePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A tag value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tag(pub String);

impl Tag {
    /// Create a new tag
    pub fn new(tag: impl Into<String>) -> Result<Self, String> {
        let tag = tag.into().trim().to_lowercase();
        if tag.is_empty() {
            return Err("Tag cannot be empty".to_string());
        }
        if tag.contains(' ') {
            return Err("Tag cannot contain spaces".to_string());
        }
        if tag.len() > 50 {
            return Err("Tag cannot be longer than 50 characters".to_string());
        }
        Ok(Self(tag))
    }

    /// Get the tag as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A project name value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectName(pub String);

impl ProjectName {
    /// Create a new project name
    pub fn new(name: impl Into<String>) -> Result<Self, String> {
        let name_str = name.into();
        let name = name_str.trim();
        if name.is_empty() {
            return Err("Project name cannot be empty".to_string());
        }
        if name.len() > 100 {
            return Err("Project name cannot be longer than 100 characters".to_string());
        }
        Ok(Self(name.to_string()))
    }

    /// Get the name as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A task title value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskTitle(pub String);

impl TaskTitle {
    /// Create a new task title
    pub fn new(title: impl Into<String>) -> Result<Self, String> {
        let title_str = title.into();
        let title = title_str.trim();
        if title.is_empty() {
            return Err("Task title cannot be empty".to_string());
        }
        if title.len() > 200 {
            return Err("Task title cannot be longer than 200 characters".to_string());
        }
        Ok(Self(title.to_string()))
    }

    /// Get the title as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TaskTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
