use crate::config::Config;
use crate::error::{FustError, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Represents a file or directory item with metadata for display
#[derive(Debug, Clone)]
pub struct FileItem {
    pub path: PathBuf,
    pub name: String,
    pub full_path_display: String,
    pub is_dir: bool,
    pub size: Option<u64>,
}

/// File operations handler with configuration
pub struct FileOperations {
    pub config: Config,
}

impl FileOperations {
    /// Create a new FileOperations instance with the given configuration
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Get the current working directory
    pub async fn get_current_dir() -> Result<PathBuf> {
        tokio::task::spawn_blocking(|| {
            std::env::current_dir().map_err(FustError::from)
        }).await.unwrap()
    }

    /// Check if a path should be ignored based on ignore patterns
    fn should_ignore_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        // Check against ignore patterns
        self.config.ignore_patterns.iter().any(|pattern| {
            // Check if pattern matches directory name
            if let Some(name) = path.file_name() {
                if name.to_string_lossy() == pattern.as_str() {
                    return true;
                }
            }
            
            // Check if pattern is contained in the full path
            path_str.contains(pattern) || path_str.contains(&format!("/{pattern}/"))
        })
    }

    /// Fast shallow search for immediate UI response
    pub async fn search_directories_fast(&self) -> Result<Vec<FileItem>> {
        let root = self.config.get_search_root();
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let mut results = Vec::new();
            let file_ops = FileOperations::new(config);

            let walker = WalkDir::new(&root)
                .follow_links(false)
                .max_depth(4)
                .into_iter()
                .filter_entry(|e| !file_ops.should_ignore_path(e.path()));

            for entry in walker.filter_map(|e| e.ok()) {
                if !entry.file_type().is_dir() {
                    continue;
                }

                let path = entry.path();

                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy().to_string();

                    let display_path = if let Ok(relative) = path.strip_prefix(&root) {
                        format!("~/{}", relative.display())
                    } else {
                        path.display().to_string()
                    };

                    results.push(FileItem {
                        path: path.to_path_buf(),
                        name: name_str,
                        full_path_display: display_path,
                        is_dir: true,
                        size: None,
                    });
                }
            }

            // Sort by path length first, then alphabetically
            results.sort_by(|a, b| {
                let len_cmp = a
                    .full_path_display
                    .matches('/')
                    .count()
                    .cmp(&b.full_path_display.matches('/').count());
                if len_cmp == std::cmp::Ordering::Equal {
                    a.full_path_display
                        .to_lowercase()
                        .cmp(&b.full_path_display.to_lowercase())
                } else {
                    len_cmp
                }
            });

            Ok(results)
        })
        .await
        .unwrap()
    }

    pub async fn search_files(&self, global: bool) -> Result<Vec<FileItem>> {
        let base_path = if global {
            self.config.get_search_root()
        } else {
            Self::get_current_dir().await?
        };

        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let mut results = Vec::new();
            let file_ops = FileOperations::new(config);

            let walker = if global {
                WalkDir::new(&base_path).follow_links(false).max_depth(8)
            } else {
                WalkDir::new(&base_path).follow_links(false).max_depth(3)
            };

            for entry in walker.into_iter().filter_map(|e| e.ok()) {
                if !entry.file_type().is_file() {
                    continue;
                }

                let path = entry.path();
                
                // Use unified filtering logic
                if file_ops.should_ignore_path(path) {
                    continue;
                }

                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy();

                    // Create display version
                    let display_path = if let Ok(relative) = path.strip_prefix(&base_path) {
                        if global {
                            format!("~/{}", relative.display())
                        } else {
                            format!("./{}", relative.display())
                        }
                    } else {
                        path.display().to_string()
                    };

                    let metadata = entry.metadata().ok();
                    results.push(FileItem {
                        path: path.to_path_buf(),
                        name: name_str.to_string(),
                        full_path_display: display_path,
                        is_dir: false,
                        size: metadata.as_ref().map(|m| m.len()),
                    });

                    if results.len() >= 10000 {
                        break;
                    }
                }
            }

            Ok(results)
        })
        .await
        .unwrap()
    }

    pub async fn open_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let editor = self.config.get_editor().ok_or(FustError::EditorNotFound)?;
        let path = path.as_ref().to_path_buf();

        tokio::task::spawn_blocking(move || {
            std::process::Command::new(editor)
                .arg(&path)
                .status()
                .map_err(FustError::from)
                .map(|_| ())
        })
        .await
        .unwrap()
    }
}
