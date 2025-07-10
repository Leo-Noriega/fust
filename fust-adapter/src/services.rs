//! Service implementations

use anyhow::Result;
use async_trait::async_trait;
use fust_domain::FilePath;
use fust_port::{
    AppConfig, ClipboardService, ConfigurationService, FileSystemService, GitCommit, GitService,
    GitStatus, UserInterfaceService,
};
use std::path::Path;
use tokio::fs;

/// File system service implementation
pub struct FileSystemServiceImpl;

#[async_trait]
impl FileSystemService for FileSystemServiceImpl {
    async fn exists(&self, path: &FilePath) -> Result<bool> {
        Ok(Path::new(path.as_str()).exists())
    }

    async fn create_directory(&self, path: &FilePath) -> Result<()> {
        fs::create_dir_all(path.as_str()).await?;
        Ok(())
    }

    async fn read_file(&self, path: &FilePath) -> Result<String> {
        let content = fs::read_to_string(path.as_str()).await?;
        Ok(content)
    }

    async fn write_file(&self, path: &FilePath, content: &str) -> Result<()> {
        fs::write(path.as_str(), content).await?;
        Ok(())
    }

    async fn list_directory(&self, path: &FilePath) -> Result<Vec<String>> {
        let mut entries = Vec::new();
        let mut read_dir = fs::read_dir(path.as_str()).await?;

        while let Some(entry) = read_dir.next_entry().await? {
            if let Ok(name) = entry.file_name().into_string() {
                entries.push(name);
            }
        }

        Ok(entries)
    }

    async fn get_file_metadata(&self, path: &FilePath) -> Result<std::fs::Metadata> {
        let metadata = fs::metadata(path.as_str()).await?;
        Ok(metadata)
    }
}

/// Git service implementation
pub struct GitServiceImpl;

#[async_trait]
impl GitService for GitServiceImpl {
    async fn is_git_repository(&self, path: &FilePath) -> Result<bool> {
        let git_dir = Path::new(path.as_str()).join(".git");
        Ok(git_dir.exists() && git_dir.is_dir())
    }

    async fn get_status(&self, _path: &FilePath) -> Result<GitStatus> {
        // TODO: Implement actual git status (git2 dependency removed due to security vulnerability)
        Ok(GitStatus {
            is_repository: true,
            current_branch: Some("main".to_string()),
            has_uncommitted_changes: false,
            has_untracked_files: false,
            ahead_count: 0,
            behind_count: 0,
        })
    }

    async fn get_current_branch(&self, _path: &FilePath) -> Result<Option<String>> {
        // TODO: Implement actual git branch (git2 dependency removed due to security vulnerability)
        Ok(Some("main".to_string()))
    }

    async fn get_remote_url(&self, _path: &FilePath) -> Result<Option<String>> {
        // TODO: Implement actual git remote (git2 dependency removed due to security vulnerability)
        Ok(None)
    }

    async fn get_commit_history(&self, _path: &FilePath, _limit: usize) -> Result<Vec<GitCommit>> {
        // TODO: Implement actual git log (git2 dependency removed due to security vulnerability)
        Ok(Vec::new())
    }
}

/// Configuration service implementation
pub struct ConfigurationServiceImpl {
    #[allow(dead_code)]
    config_path: String,
}

impl ConfigurationServiceImpl {
    /// Create a new configuration service
    pub fn new(config_path: String) -> Self {
        Self { config_path }
    }
}

#[async_trait]
impl ConfigurationService for ConfigurationServiceImpl {
    async fn load(&self) -> Result<AppConfig> {
        // TODO: Implement actual configuration loading
        Ok(AppConfig::default())
    }

    async fn save(&self, _config: &AppConfig) -> Result<()> {
        // TODO: Implement actual configuration saving
        Ok(())
    }

    async fn get_value(&self, _key: &str) -> Result<Option<String>> {
        // TODO: Implement actual configuration get
        Ok(None)
    }

    async fn set_value(&self, _key: &str, _value: &str) -> Result<()> {
        // TODO: Implement actual configuration set
        Ok(())
    }
}

/// Clipboard service implementation
pub struct ClipboardServiceImpl;

#[async_trait]
impl ClipboardService for ClipboardServiceImpl {
    async fn copy_text(&self, text: &str) -> Result<()> {
        // TODO: Implement actual clipboard copy using arboard
        println!("Copying to clipboard: {}", text);
        Ok(())
    }

    async fn get_text(&self) -> Result<Option<String>> {
        // TODO: Implement actual clipboard get using arboard
        Ok(None)
    }
}

/// User interface service implementation
pub struct UserInterfaceServiceImpl;

#[async_trait]
impl UserInterfaceService for UserInterfaceServiceImpl {
    async fn show_selection(&self, items: &[String]) -> Result<Option<String>> {
        // TODO: Implement actual selection using skim
        if items.is_empty() {
            return Ok(None);
        }

        // For now, just return the first item
        Ok(Some(items[0].clone()))
    }

    async fn show_input(&self, prompt: &str) -> Result<Option<String>> {
        // TODO: Implement actual input prompt
        println!("{}", prompt);
        Ok(None)
    }

    async fn show_confirmation(&self, message: &str) -> Result<bool> {
        // TODO: Implement actual confirmation dialog
        println!("{} (y/N)", message);
        Ok(false)
    }

    async fn display_message(&self, message: &str) -> Result<()> {
        println!("{}", message);
        Ok(())
    }
}
