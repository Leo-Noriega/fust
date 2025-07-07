use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub global_file_search: bool,
    pub ignore_patterns: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            global_file_search: false,
            ignore_patterns: vec![
                // Version control
                ".git".to_string(),
                ".svn".to_string(),
                ".hg".to_string(),
                
                // Build directories and caches
                "node_modules".to_string(),
                "target".to_string(),
                "build".to_string(),
                "dist".to_string(),
                ".next".to_string(),
                ".nuxt".to_string(),
                "__pycache__".to_string(),
                ".pytest_cache".to_string(),
                ".mypy_cache".to_string(),
                
                // System directories and files
                ".DS_Store".to_string(),
                ".Trash".to_string(),
                "Trash".to_string(),
                ".cache".to_string(),
                ".tmp".to_string(),
                "tmp".to_string(),
                
                // Development environments
                ".venv".to_string(),
                "venv".to_string(),
                ".virtualenv".to_string(),
                "virtualenv".to_string(),
                ".conda".to_string(),
                "anaconda3".to_string(),
                "miniconda3".to_string(),
                
                // Docker and containers
                ".docker".to_string(),
                "docker".to_string(),
                
                // IDEs and editors
                ".vscode".to_string(),
                ".idea".to_string(),
                ".eclipse".to_string(),
                
                // Logs and temporary files
                "logs".to_string(),
                "log".to_string(),
                ".log".to_string(),
                
                // Package managers
                ".npm".to_string(),
                ".yarn".to_string(),
                ".pnpm".to_string(),
                "bower_components".to_string(),
                
                // Language-specific
                "Cargo.lock".to_string(),  // Rust
                "vendor".to_string(),      // Go
                ".m2".to_string(),         // Java Maven
                ".gradle".to_string(),     // Java Gradle
                
                // macOS specific
                ".fseventsd".to_string(),
                ".Spotlight-V100".to_string(),
                ".TemporaryItems".to_string(),
                ".Trashes".to_string(),
                
                // Large system directories
                "System".to_string(),
                "usr".to_string(),
                "var".to_string(),
                "proc".to_string(),
                "dev".to_string(),
                "sys".to_string(),
            ],
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_editor(&self) -> Option<String> {
        std::env::var("VISUAL")
            .ok()
            .or_else(|| std::env::var("EDITOR").ok())
            .or_else(|| {
                for editor in &["nvim", "vim", "nano", "code"] {
                    if std::process::Command::new("which")
                        .arg(editor)
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false)
                    {
                        return Some(editor.to_string());
                    }
                }
                None
            })
    }

    pub fn get_search_root(&self) -> PathBuf {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
    }
}
