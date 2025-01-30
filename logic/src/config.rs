use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid value provided")]
    InvalidValue,
    #[error("Failed to read config file")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to serialize config file")]
    SerializeError(#[from] serde_json::Error),
}

/// The configuration properties for this application
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub cli_print_style: PrintStyle,

    #[serde(default)]
    pub cli_display_limit: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cli_print_style: PrintStyle::All,
            cli_display_limit: 10,
        }
    }
}

/// The printing styles supported for commands in the CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrintStyle {
    /// Display the command, tag, and notes
    All,

    /// Only display the command
    Command,
}

impl FromStr for PrintStyle {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match str::to_lowercase(s).as_ref() {
            "all" => Ok(PrintStyle::All),
            "command" => Ok(PrintStyle::Command),
            _ => Err(ConfigError::InvalidValue),
        }
    }
}

impl Default for PrintStyle {
    fn default() -> Self {
        PrintStyle::All
    }
}

impl Config {
    pub fn read() -> Result<Self, ConfigError> {
        let config_path = Config::config_file_path()?;
        let config_content = fs::read_to_string(config_path).unwrap_or_else(|_| String::from("{}"));

        let config = serde_json::from_str(&config_content)?;

        Ok(config)
    }

    pub fn write(&self) -> Result<(), ConfigError> {
        let config_path = Config::config_file_path()?;
        let config_content =
            serde_json::to_string_pretty(self).map_err(ConfigError::SerializeError)?;
        fs::write(config_path, config_content)?;
        Ok(())
    }

    fn config_file_path() -> Result<PathBuf, ConfigError> {
        let mut config_dir = dirs::config_dir().ok_or_else(|| {
            ConfigError::ReadError(std::io::Error::new(
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
