use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid value provided: {0}")]
    InvalidValue(String),
    #[error("Failed to locate config directory")]
    Directory(#[from] std::io::Error),
    #[error("Failed to serialize config file")]
    Serialize(#[from] serde_json::Error),
}

/// Configuration structure for reading/writing JSON
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(default)]
    pub cli_print_style: CliPrintStyle,

    #[serde(default)]
    pub cli_display_limit: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cli_print_style: CliPrintStyle::All,
            cli_display_limit: 10,
        }
    }
}

/// Arguments for setting the CLI config
#[derive(Debug, Clone)]
pub enum ConfigProperty {
    CliPrintStyle(CliPrintStyle),
    CliDisplayLimit(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum CliPrintStyle {
    #[default]
    All,
    CommandsOnly,
}

impl Config {
    pub fn read() -> Result<Config, ConfigError> {
        let config_path = Config::config_file_path()?;
        let config_content = fs::read_to_string(&config_path).unwrap_or_else(|_| "{}".to_string());
        let config = serde_json::from_str(&config_content).unwrap_or_default();

        Ok(config)
    }

    pub fn write(&self) -> Result<(), ConfigError> {
        let config_path = Config::config_file_path()?;
        let config_content = serde_json::to_string_pretty(self).map_err(ConfigError::Serialize)?;
        fs::write(config_path, config_content)?;

        Ok(())
    }

    fn config_file_path() -> Result<PathBuf, ConfigError> {
        let mut config_dir = dirs::config_dir().ok_or_else(|| {
            ConfigError::Directory(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find config directory",
            ))
        })?;
        config_dir.push("cmdstack");
        fs::create_dir_all(&config_dir)?;
        config_dir.push("config.json");
        Ok(config_dir)
    }
}
