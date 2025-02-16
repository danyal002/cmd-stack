mod args;
mod command;
pub mod outputs;
pub mod utils;

use args::{CmdStackArgs, Command};
use clap::Parser;
use command::{
    add_command::HandleAddError, delete_command::HandleDeleteError,
    search_command::HandleSearchError, update_command::HandleUpdateError,
};
use log::{error, LevelFilter, SetLoggerError};
use log4rs::append::file::FileAppender;
use log4rs::config::runtime::ConfigErrors;
use log4rs::config::{Appender, Config, Root};
use logic::Logic;
use outputs::{ErrorOutput, Output};
use thiserror::Error;

#[derive(Error, Debug)]
enum LoggerInitializationError {
    #[error("Could not get the log file path")]
    LogPath(String),

    #[error("Could not create the log file")]
    CreatingLogFile(#[from] std::io::Error),

    #[error("Could not create config")]
    CreatingLogConfig(#[from] ConfigErrors),

    #[error("Could not initialize logger")]
    InitializingLogger(#[from] SetLoggerError),
}

/// Set up logging for CLI
///
/// If we are doing local development, set up environment logger.
/// Otherwise (if the app was built with the `--release` flag), send logs to a file in the user's file statem.
fn initialize_logger() -> Result<(), LoggerInitializationError> {
    if cfg!(debug_assertions) {
        // Set up environment logger
        env_logger::Builder::new()
            .filter(None, LevelFilter::Info)
            .init();
    } else {
        // Log file path: $HOME/.config/cmdstack/cmdstack.log
        let config_dir = match dirs::config_dir() {
            Some(dir) => match dir.to_str() {
                Some(path) => path.to_string(),
                None => {
                    return Err(LoggerInitializationError::LogPath(
                        "Could not convert config directory to string".to_string(),
                    ));
                }
            },
            None => {
                return Err(LoggerInitializationError::LogPath(
                    "Could not get config directory".to_string(),
                ));
            }
        };
        let logfile_path = config_dir + "/cmdstack/cmdstack.log";

        let logfile = FileAppender::builder().build(logfile_path)?;

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

        log4rs::init_config(config)?;
    }

    Ok(())
}

pub struct Cli {
    logic: Logic,
}

fn main() {
    let _ = initialize_logger().map_err(|_| {
        ErrorOutput::Logger.print();
        std::process::exit(1);
    });

    // TODO: Instead of panicking, show a user-friendly error, log and quit.
    let logic = Logic::try_default().map_err(|e| panic!("{}", e)).unwrap();
    let mut cli = Cli { logic };

    // Configure inquire
    inquire::set_global_render_config(inquire::ui::RenderConfig {
        prompt: inquire::ui::StyleSheet::default().with_fg(inquire::ui::Color::LightBlue),
        ..Default::default()
    });

    let args = CmdStackArgs::parse();

    match args.command {
        Command::Add(add_args) => match cli.handle_add_command(add_args) {
            Ok(_) => Output::AddCommandSuccess.print(),
            Err(e) => {
                match e {
                    HandleAddError::Inquire(_) => ErrorOutput::UserInput.print(),
                    _ => ErrorOutput::AddCommand.print(),
                };
                error!("Error occurred while adding command: {:?}", e);
            }
        },
        Command::Update(update_args) => match cli.handle_update_command(update_args) {
            Ok(_) => Output::UpdateCommandSuccess.print(),
            Err(e) => {
                match e {
                    HandleUpdateError::NoCommandFound => Output::NoCommandsFound.print(),
                    HandleUpdateError::Inquire(_) => ErrorOutput::UserInput.print(),
                    HandleUpdateError::SelectCommand(_) => ErrorOutput::UserInput.print(),
                    _ => ErrorOutput::UpdateCommand.print(),
                };
                error!("Error occurred while updating command: {:?}", e);
            }
        },
        Command::Delete(delete_args) => match cli.handle_delete_command(delete_args) {
            Ok(_) => Output::DeleteCommandSuccess.print(),
            Err(e) => {
                match e {
                    HandleDeleteError::NoCommandsFound => Output::NoCommandsFound.print(),
                    HandleDeleteError::Inquire(_) => ErrorOutput::UserInput.print(),
                    HandleDeleteError::SelectCommand(_) => ErrorOutput::UserInput.print(),
                    _ => ErrorOutput::DeleteCommand.print(),
                };
                error!("Error occurred while deleting command: {:?}", e);
            }
        },
        Command::Search(search_args) => match cli.handle_search_command(search_args) {
            Ok(_) => Output::CommandCopiedToClipboard.print(),
            Err(e) => {
                match e {
                    HandleSearchError::NoCommandFound => Output::NoCommandsFound.print(),
                    HandleSearchError::Inquire(_) => ErrorOutput::UserInput.print(),
                    HandleSearchError::SelectCommand(_) => ErrorOutput::UserInput.print(),
                    _ => ErrorOutput::SearchCommand.print(),
                };
                error!("Error occurred while searching commands: {:?}", e);
            }
        },
        Command::Export(export_args) => match cli.handle_export_command(export_args) {
            Ok(file_path) => Output::ExportCommandsSuccess(&file_path).print(),
            Err(e) => {
                ErrorOutput::Export.print();
                error!("Error occurred while exporting commands: {:?}", e);
            }
        },
        Command::Import(import_args) => match cli.handle_import_command(import_args) {
            Ok((num, file_path)) => Output::ImportCommandsSuccess(num, &file_path).print(),
            Err(e) => {
                ErrorOutput::Import.print();
                error!("Error occurred while importing commands: {:?}", e);
            }
        },
        Command::Config(config_args) => match cli.handle_config_command(config_args) {
            Ok(()) => Output::ConfigUpdate.print(),
            Err(e) => {
                ErrorOutput::Config.print();
                error!("Error occurred while updating config: {:?}", e);
            }
        },
    }
}
