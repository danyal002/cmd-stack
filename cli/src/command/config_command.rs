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
    #[error("Failed to read config: {0}")]
    ReadConfig(#[from] logic::config::ConfigReadError),
    #[error("Failed to write config: {0}")]
    WriteConfig(#[from] logic::config::ConfigWriteError),
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help(true))]
pub enum ConfigArgs {
    /// Modify cli print style
    CliPrintStyle(CliPrintStyleArgs),

    /// Modify cli display limit
    CliDisplayLimit(CliDisplayLimitArgs),

    /// Modify string parameter min/max limits
    ParamStringLength(ParamStringArgs),

    /// Modify integer parameter min/max limits
    ParamIntRange(ParamIntArgs),
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

#[derive(Debug, Args, Validate)]
#[command(arg_required_else_help(true))]
pub struct ParamStringArgs {
    #[validate(range(min = 1, message = "param-string-min must be at least 1"))]
    /// The minimum length for string parameters (min. 1)
    pub min: u32,

    #[validate(range(min = 1, message = "param-string-max must be at least 1"))]
    /// The maximum length for string parameters (min. 1)
    pub max: u32,
}

#[derive(Debug, Args, Validate)]
#[command(arg_required_else_help(true))]
pub struct ParamIntArgs {
    /// The minimum value for integer parameters
    pub min: i32,

    /// The maximum value for integer parameters
    pub max: i32,
}

impl Cli {
    /// Handles the config modification command
    pub fn handle_config_command(&mut self, config_args: ConfigArgs) -> Result<(), ConfigError> {
        match config_args {
            ConfigArgs::CliPrintStyle(cli_print_style_args) => {
                self.logic.config.cli_print_style = cli_print_style_args.style.into()
            }
            ConfigArgs::CliDisplayLimit(cli_display_limit_args) => {
                cli_display_limit_args
                    .validate()
                    .map_err(|e| ConfigError::InvalidValue(e.to_string()))?;

                self.logic.config.cli_display_limit = cli_display_limit_args.value
            }
            ConfigArgs::ParamStringLength(param_string_args) => {
                param_string_args
                    .validate()
                    .map_err(|e| ConfigError::InvalidValue(e.to_string()))?;

                if param_string_args.min > param_string_args.max {
                    return Err(ConfigError::InvalidValue(format!(
                        "param-string-min ({}) cannot be greater than param-string-max ({})",
                        param_string_args.min, param_string_args.max
                    )));
                }

                self.logic.config.param_string_min = param_string_args.min;
                self.logic.config.param_string_max = param_string_args.max;
            }
            ConfigArgs::ParamIntRange(param_int_args) => {
                if param_int_args.min > param_int_args.max {
                    return Err(ConfigError::InvalidValue(format!(
                        "param-int-min ({}) cannot be greater than param-int-max ({})",
                        param_int_args.min, param_int_args.max
                    )));
                }

                self.logic.config.param_int_min = param_int_args.min;
                self.logic.config.param_int_max = param_int_args.max;
            }
        }
        Ok(self.logic.config.write()?)
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
