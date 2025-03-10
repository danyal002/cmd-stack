use clap::{Args, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

use crate::Cli;

#[derive(Debug, Error)]
pub enum ConfigError {
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
    ParamStringLength(ParamStringLengthArgs),

    /// Modify integer parameter min/max limits
    ParamIntRange(ParamIntRangeArgs),

    /// Modify application theme
    Theme(ApplicationThemeArgs),
}

#[derive(Debug, Args)]
#[command(arg_required_else_help(true))]
pub struct ApplicationThemeArgs {
    /// The application theme
    #[clap(value_enum)]
    pub theme: ApplicationTheme,
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize, Default)]
pub enum ApplicationTheme {
    #[default]
    System,
    Dark,
    Light,
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
pub struct ParamStringLengthArgs {
    /// The minimum length for string parameters (min. 1)
    #[validate(range(min = 1, message = "param-string-length-min must be at least 1"))]
    #[arg(long = "min")]
    pub min: Option<u32>,

    /// The maximum length for string parameters (min. 1)
    #[validate(range(min = 1, message = "param-string-length-max must be at least 1"))]
    #[arg(long = "max")]
    pub max: Option<u32>,
}

#[derive(Debug, Args, Validate)]
#[command(arg_required_else_help(true))]
pub struct ParamIntRangeArgs {
    /// The minimum value for integer parameters
    #[arg(long = "min")]
    pub min: Option<i32>,

    /// The maximum value for integer parameters
    #[arg(long = "max")]
    pub max: Option<i32>,
}

impl Cli {
    /// Handles the config modification command
    pub fn handle_config_command(&mut self, config_args: ConfigArgs) -> Result<(), ConfigError> {
        match config_args {
            ConfigArgs::Theme(theme_args) => {
                self.logic.config.application_theme = theme_args.theme.into()
            }
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
                if param_string_args.min.is_none() && param_string_args.max.is_none() {
                    return Err(ConfigError::InvalidValue(
                        "No value(s) provided for param-string-length".to_string(),
                    ));
                }

                param_string_args
                    .validate()
                    .map_err(|e| ConfigError::InvalidValue(e.to_string()))?;

                let min = param_string_args
                    .min
                    .unwrap_or(self.logic.config.param_string_length_min);

                let max = param_string_args
                    .max
                    .unwrap_or(self.logic.config.param_string_length_max);

                if min > max {
                    return Err(ConfigError::InvalidValue(format!(
                        "param-string-length-min ({}) cannot be greater than param-string-length-max ({})",
                        min, max
                    )));
                }

                self.logic.config.param_string_length_min = min;
                self.logic.config.param_string_length_max = max;
            }
            ConfigArgs::ParamIntRange(param_int_args) => {
                if param_int_args.min.is_none() && param_int_args.max.is_none() {
                    return Err(ConfigError::InvalidValue(
                        "No value(s) provided for param-int-range".to_string(),
                    ));
                }

                let min = param_int_args
                    .min
                    .unwrap_or(self.logic.config.param_int_range_min);

                let max = param_int_args
                    .max
                    .unwrap_or(self.logic.config.param_int_range_min);

                if min > max {
                    return Err(ConfigError::InvalidValue(format!(
                        "param-int-range-min ({}) cannot be greater than param-int-range-max ({})",
                        min, max
                    )));
                }

                self.logic.config.param_int_range_min = min;
                self.logic.config.param_int_range_max = max;
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

impl From<ApplicationTheme> for logic::config::ApplicationTheme {
    fn from(item: ApplicationTheme) -> Self {
        match item {
            ApplicationTheme::System => Self::System,
            ApplicationTheme::Dark => Self::Dark,
            ApplicationTheme::Light => Self::Light,
        }
    }
}
