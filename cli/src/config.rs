use crate::args::{ConfigArgs, PrintStyle, SearchAndPrintArgs};
use crate::outputs::{format_output, spacing};
use inquire::InquireError;
use inquire::{validator::Validation, CustomType, Select};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to serialize config file")]
    SerializeError(#[source] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_print_style")]
    pub print_style: PrintStyle,

    #[serde(default = "default_display_limit")]
    pub display_limit: i32,
}

/// Default value for print style
fn default_print_style() -> PrintStyle {
    PrintStyle::All
}

/// Default value for display limit
fn default_display_limit() -> i32 {
    10
}

impl Default for Config {
    fn default() -> Self {
        Self {
            print_style: default_print_style(),
            display_limit: default_display_limit(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Config::config_file_path()?;
        let config_content = fs::read_to_string(config_path).unwrap_or_else(|_| String::from("{}"));

        let config: Config =
            serde_json::from_str(&config_content).unwrap_or_else(|_| Config::default());

        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
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

    /// Merges CLI args with config values, prioritizing CLI args if provided
    pub fn merge_config_with_search_and_print_args(
        &self,
        args: &SearchAndPrintArgs,
    ) -> (PrintStyle, i32) {
        let print_style = args.print_style.clone().unwrap_or(self.print_style.clone());
        let display_limit = args.display_limit.unwrap_or(self.display_limit);

        (print_style, display_limit)
    }
}

#[derive(Error, Debug)]
pub enum HandleConfigError {
    #[error("Failed to load config")]
    LoadConfig(#[source] ConfigError),
    #[error("Failed to save config")]
    SaveConfig(#[source] ConfigError),
    #[error("Failed to get user input")]
    Inquire(#[from] InquireError),
}

pub fn handle_config_command(args: ConfigArgs) -> Result<(), HandleConfigError> {
    let mut config = Config::load().map_err(HandleConfigError::LoadConfig)?;

    config.print_style = if let Some(print_style) = args.print_style.clone() {
        print_style
    } else {
        spacing();
        Select::new(
            &format_output("<bold>Print Style:</bold>"),
            vec![PrintStyle::All, PrintStyle::Command],
        )
        .prompt()?
    };

    config.display_limit = if let Some(display_limit) = args.display_limit {
        display_limit
    } else {
        // Print spacing only if the print style input was not displayed
        if args.print_style.is_some() {
            spacing();
        }
        CustomType::new(&format_output("<bold>Enter display limit</bold> <italics>(Must be greater than 0)</italics><bold>:</bold>"))
            .with_validator(|&val: &i32| {
                if val > 0 {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid("Display limit must be greater than 0".into()))
                }
            })
            .with_default(config.display_limit)
            .prompt()?
    };

    config.save().map_err(HandleConfigError::SaveConfig)?;

    Ok(())
}
