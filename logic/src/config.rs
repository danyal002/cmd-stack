use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigWriteError {
    #[error("Invalid value provided: {0}")]
    InvalidValue(String),
    #[error("Failed to locate default config directory")]
    DefaultConfigDirectory,
    #[error("Failed to write config to file: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to deserialize config file: {0}")]
    Deserialize(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum ConfigReadError {
    #[error("Invalid value provided: {0}")]
    InvalidValue(String),
    #[error("Failed to locate default config directory")]
    DefaultConfigDirectory,
    #[error("Failed to read from config file: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to serialize config file: {0}")]
    Serialize(#[from] serde_json::Error),
}

/// Configuration structure for reading/writing JSON
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(default)]
pub struct Config {
    pub cli_print_style: CliPrintStyle,
    pub cli_display_limit: u32,
    pub param_string_length_min: u32,
    pub param_string_length_max: u32,
    pub param_int_range_min: i32,
    pub param_int_range_max: i32,
    pub application_theme: ApplicationTheme,
    pub default_terminal: UiDefaultTerminal,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cli_print_style: CliPrintStyle::default(),
            cli_display_limit: 10,
            param_string_length_min: 5,
            param_string_length_max: 10,
            param_int_range_min: 5,
            param_int_range_max: 10,
            application_theme: ApplicationTheme::default(),
            default_terminal: UiDefaultTerminal::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Copy)]
pub enum ApplicationTheme {
    #[default]
    System,
    Dark,
    Light,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Copy)]
pub enum CliPrintStyle {
    #[default]
    All,
    CommandsOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Copy, PartialEq)]
pub enum UiDefaultTerminal {
    Iterm,
    #[default]
    Terminal,
}

impl Config {
    pub fn read() -> Result<Config, ConfigReadError> {
        let config_path =
            Config::default_config_file_path()?.ok_or(ConfigReadError::DefaultConfigDirectory)?;
        let config_content = fs::read_to_string(&config_path).unwrap_or_else(|_| "{}".to_string());
        let config: Config =
            serde_json::from_str(&config_content).map_err(ConfigReadError::Serialize)?;

        Ok(config)
    }

    pub fn write(&self) -> Result<(), ConfigWriteError> {
        let config_path =
            Config::default_config_file_path()?.ok_or(ConfigWriteError::DefaultConfigDirectory)?;
        let config_file_content =
            serde_json::to_string_pretty(self).map_err(ConfigWriteError::Deserialize)?;

        Ok(fs::write(config_path, config_file_content)?)
    }

    fn default_config_file_path() -> Result<Option<PathBuf>, io::Error> {
        if let Some(mut path) = dirs::config_dir() {
            path.push("cmdstack");
            // If we fail to create the directory, return an error with the path
            if let Err(e) = fs::create_dir_all(&path) {
                return Err(io::Error::new(
                    e.kind(),
                    format!("Failed to create config directory: {}", path.display()),
                ));
            }
            path.push("config.json");

            return Ok(Some(path));
        }
        Ok(None)
    }
}
