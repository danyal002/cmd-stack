use crate::args::ConfigArgs;
use logic::config::PrintStyle;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Unknown property provided")]
    UnknownProperty,
    #[error("Invalid native value provided")]
    InvalidValue,
    #[error("Failed to read config")]
    ReadConfig(#[source] logic::config::ConfigError),
    #[error("Failed to read config")]
    WriteConfig(#[source] logic::config::ConfigError),
}

// String - key
// String - value
pub fn handle_config_command(config_args: ConfigArgs) -> Result<(), ConfigError> {
    let mut config = logic::config::Config::read().map_err(ConfigError::ReadConfig)?;

    match config_args.property.as_ref() {
        "cli_print_style" => {
            let val: PrintStyle = config_args
                .value
                .parse()
                .map_err(|_| ConfigError::InvalidValue)?;
            config.cli_print_style = val;
        }
        "cli_display_limit" => {
            let val: u32 = config_args
                .value
                .parse()
                .map_err(|_| ConfigError::InvalidValue)?;
            config.cli_display_limit = val;
        }
        _ => {
            return Err(ConfigError::UnknownProperty);
        }
    }

    config.write().map_err(ConfigError::WriteConfig)?;

    Ok(())
}
