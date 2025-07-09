//! Configuration management

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    /// Workspace path
    pub workspace_path: Option<PathBuf>,
    /// Default project
    pub default_project: Option<String>,
    /// Theme
    pub theme: String,
    /// Auto save
    pub auto_save: bool,
    /// Notifications enabled
    pub notifications_enabled: bool,
}

impl Default for CliConfig {
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

impl CliConfig {
    /// Load configuration from file
    pub fn load(config_path: Option<&str>) -> Result<Self> {
        let config_path = config_path
            .map(PathBuf::from)
            .unwrap_or_else(|| get_default_config_path());

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: CliConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// Save configuration to file
    pub fn save(&self, config_path: Option<&str>) -> Result<()> {
        let config_path = config_path
            .map(PathBuf::from)
            .unwrap_or_else(|| get_default_config_path());

        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    /// Get configuration value
    pub fn get_value(&self, key: &str) -> Option<String> {
        match key {
            "workspace_path" => self.workspace_path.as_ref().map(|p| p.to_string_lossy().to_string()),
            "default_project" => self.default_project.clone(),
            "theme" => Some(self.theme.clone()),
            "auto_save" => Some(self.auto_save.to_string()),
            "notifications_enabled" => Some(self.notifications_enabled.to_string()),
            _ => None,
        }
    }

    /// Set configuration value
    pub fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "workspace_path" => {
                self.workspace_path = Some(PathBuf::from(value));
            }
            "default_project" => {
                self.default_project = Some(value.to_string());
            }
            "theme" => {
                self.theme = value.to_string();
            }
            "auto_save" => {
                self.auto_save = value.parse().map_err(|_| {
                    anyhow::anyhow!("Invalid boolean value for auto_save: {}", value)
                })?;
            }
            "notifications_enabled" => {
                self.notifications_enabled = value.parse().map_err(|_| {
                    anyhow::anyhow!("Invalid boolean value for notifications_enabled: {}", value)
                })?;
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown configuration key: {}", key));
            }
        }
        Ok(())
    }
}

/// Get default configuration path
fn get_default_config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push("fust");
    config_dir.push("config.json");
    config_dir
}

/// Load configuration
pub fn load_config(config_path: Option<&str>) -> Result<CliConfig> {
    CliConfig::load(config_path)
} 