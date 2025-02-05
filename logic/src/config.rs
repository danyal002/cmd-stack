use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigWriteError {
    #[error("Invalid value provided: {0}")]
    InvalidValue(String),
    #[error("Failed to locate config directory")]
    ConfigPath,
    #[error("Failed to write config to file: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to deserialize config file")]
    Deserialize(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum ConfigReadError {
    #[error("Invalid value provided: {0}")]
    InvalidValue(String),
    #[error("Failed to locate config directory")]
    ConfigPath,
    #[error("Failed to read from config file: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to serialize config file")]
    Serialize(#[from] serde_json::Error),
}

/// Configuration structure for reading/writing JSON
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub cli_print_style: CliPrintStyle,
    pub cli_display_limit: u32,
    pub param_string_min: u32,
    pub param_string_max: u32,
    pub param_int_min: i32,
    pub param_int_max: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cli_print_style: CliPrintStyle::default(),
            cli_display_limit: 10,
            param_string_min: 5,
            param_string_max: 10,
            param_int_min: 5,
            param_int_max: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum CliPrintStyle {
    #[default]
    All,
    CommandsOnly,
}

impl Config {
    pub fn read() -> Result<Config, ConfigReadError> {
        let config_path = Config::default_config_file_path()?.ok_or(ConfigReadError::ConfigPath)?;
        let config_content = fs::read_to_string(&config_path).unwrap_or_else(|_| "{}".to_string());
        let config: Config =
            serde_json::from_str(&config_content).map_err(ConfigReadError::Serialize)?;

        Ok(config)
    }

    pub fn write(&self) -> Result<(), ConfigWriteError> {
        let config_path =
            Config::default_config_file_path()?.ok_or(ConfigWriteError::ConfigPath)?;
        let config_file_content =
            serde_json::to_string_pretty(self).map_err(ConfigWriteError::Deserialize)?;

        Ok(fs::write(config_path, config_file_content)?)
    }

    fn default_config_file_path() -> Result<Option<PathBuf>, io::Error> {
        if let Some(mut path) = dirs::config_dir() {
            path.push("cmdstack");
            fs::create_dir_all(path.as_path())?;
            path.push("config.json");

            return Ok(Some(path));
        }
        Ok(None)
    }
}
