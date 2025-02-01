use clap::{Args, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

use crate::Cli;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Unknown property provided")]
    UnknownProperty,
    #[error("Invalid value provided: {0}")]
    InvalidValue(String),
    #[error("Failed to read config")]
    ReadConfig(#[source] logic::config::ConfigError),
    #[error("Failed to write config")]
    WriteConfig(#[source] logic::config::ConfigError),
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help(true))]
pub enum ConfigArgs {
    /// Modify cli print style
    CliPrintStyle(CliPrintStyleArgs),

    /// Modify cli display limit
    CliDisplayLimit(CliDisplayLimitArgs),
}

#[derive(Debug, Args)]
#[command(arg_required_else_help(true))]
pub struct CliPrintStyleArgs {
    /// The new print style
    #[clap(value_enum)]
    pub style: CliPrintStyle,
}

#[derive(Debug, Args, Validate)]
#[command(arg_required_else_help(true))]
pub struct CliDisplayLimitArgs {
    #[validate(range(min = 1, message = "cli-display-limit must be at least 1"))]
    /// The new display limit (min. 1)
    pub value: u32,
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize, Default)]
pub enum CliPrintStyle {
    #[default]
    All,
    CommandsOnly,
}

impl Cli {
    /// Handles the config modification command
    pub fn handle_config_command(&self, config_args: ConfigArgs) -> Result<(), ConfigError> {
        let mut config = logic::config::Config::read().map_err(ConfigError::ReadConfig)?;

        match config_args {
            ConfigArgs::CliPrintStyle(cli_print_style_args) => {
                config.cli_print_style = cli_print_style_args.style.into()
            }
            ConfigArgs::CliDisplayLimit(cli_display_limit_args) => {
                cli_display_limit_args
                    .validate()
                    .map_err(|e| ConfigError::InvalidValue(e.to_string()))?;

                config.cli_display_limit = cli_display_limit_args.value
            }
        }
        config.write().map_err(ConfigError::WriteConfig)
    }
}

impl From<CliPrintStyle> for logic::config::CliPrintStyle {
    fn from(item: CliPrintStyle) -> Self {
        match item {
            CliPrintStyle::All => Self::All,
            CliPrintStyle::CommandsOnly => Self::CommandsOnly,
        }
    }
}
