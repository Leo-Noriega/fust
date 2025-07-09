//! Service interfaces

use async_trait::async_trait;
use fust_domain::FilePath;

/// File system service interface
#[async_trait]
pub trait FileSystemService: Send + Sync {
    /// Check if a path exists
    async fn exists(&self, path: &FilePath) -> anyhow::Result<bool>;

    /// Create a directory
    async fn create_directory(&self, path: &FilePath) -> anyhow::Result<()>;

    /// Read file contents
    async fn read_file(&self, path: &FilePath) -> anyhow::Result<String>;

    /// Write file contents
    async fn write_file(&self, path: &FilePath, content: &str) -> anyhow::Result<()>;

    /// List directory contents
    async fn list_directory(&self, path: &FilePath) -> anyhow::Result<Vec<String>>;

    /// Get file metadata
    async fn get_file_metadata(&self, path: &FilePath) -> anyhow::Result<std::fs::Metadata>;
}

/// Git service interface
#[async_trait]
pub trait GitService: Send + Sync {
    /// Check if a path is a git repository
    async fn is_git_repository(&self, path: &FilePath) -> anyhow::Result<bool>;

    /// Get git status
    async fn get_status(&self, path: &FilePath) -> anyhow::Result<GitStatus>;

    /// Get current branch
    async fn get_current_branch(&self, path: &FilePath) -> anyhow::Result<Option<String>>;

    /// Get remote URL
    async fn get_remote_url(&self, path: &FilePath) -> anyhow::Result<Option<String>>;

    /// Get commit history
    async fn get_commit_history(&self, path: &FilePath, limit: usize) -> anyhow::Result<Vec<GitCommit>>;
}

/// Configuration service interface
#[async_trait]
pub trait ConfigurationService: Send + Sync {
    /// Load configuration
    async fn load(&self) -> anyhow::Result<AppConfig>;

    /// Save configuration
    async fn save(&self, config: &AppConfig) -> anyhow::Result<()>;

    /// Get configuration value
    async fn get_value(&self, key: &str) -> anyhow::Result<Option<String>>;

    /// Set configuration value
    async fn set_value(&self, key: &str, value: &str) -> anyhow::Result<()>;
}

/// Clipboard service interface
#[async_trait]
pub trait ClipboardService: Send + Sync {
    /// Copy text to clipboard
    async fn copy_text(&self, text: &str) -> anyhow::Result<()>;

    /// Get text from clipboard
    async fn get_text(&self) -> anyhow::Result<Option<String>>;
}

/// User interface service
#[async_trait]
pub trait UserInterfaceService: Send + Sync {
    /// Show interactive selection
    async fn show_selection(&self, items: &[String]) -> anyhow::Result<Option<String>>;

    /// Show input prompt
    async fn show_input(&self, prompt: &str) -> anyhow::Result<Option<String>>;

    /// Show confirmation dialog
    async fn show_confirmation(&self, message: &str) -> anyhow::Result<bool>;

    /// Display message
    async fn display_message(&self, message: &str) -> anyhow::Result<()>;
}

/// Git status information
#[derive(Debug, Clone)]
pub struct GitStatus {
    pub is_repository: bool,
    pub current_branch: Option<String>,
    pub has_uncommitted_changes: bool,
    pub has_untracked_files: bool,
    pub ahead_count: usize,
    pub behind_count: usize,
}

/// Git commit information
#[derive(Debug, Clone)]
pub struct GitCommit {
    pub hash: String,
    pub author: String,
    pub message: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

/// Application configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub workspace_path: Option<String>,
    pub default_project: Option<String>,
    pub theme: String,
    pub auto_save: bool,
    pub notifications_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            workspace_path: None,
            default_project: None,
            theme: "default".to_string(),
            auto_save: true,
            notifications_enabled: true,
        }
    }
} 