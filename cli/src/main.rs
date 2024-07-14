//! # CLI
//!
//! This crate handles user interaction in the terminal

mod args;
mod command;
mod import_export;
pub mod outputs;
mod param;

use args::{CmdStackArgs, Command};
use clap::Parser;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use thiserror::Error;

#[derive(Error, Debug)]
enum LoggerInitializationError {
    #[error("Could not get the log file path")]
    LogPath(String),

    #[error("Could not create the log file")]
    CreatingLogFile(#[from] std::io::Error),

    #[error("Could not create config")]
    CreatingLogConfig,

    #[error("Could not initialize logger")]
    InitializingLogger,
}

/// Set up logging. Can log in any crate using the log crate
///
/// If we are doing local development, set up environment logger.
/// Otherwise (if the app was built with the --release flag), send logs to a file in the user's file statem.
fn initialize_logger() -> Result<(), LoggerInitializationError> {
    if cfg!(debug_assertions) {
        // Set up environment logger
        env_logger::Builder::new()
            .filter(None, LevelFilter::Info)
            .init();
    } else {
        // Log file path: $HOME/.config/cmdstack/cmdstack.log
        let home_dir = match dirs::home_dir() {
            Some(dir) => match dir.to_str() {
                Some(path) => path.to_string(),
                None => {
                    return Err(LoggerInitializationError::LogPath(
                        "Could not convert home directory to string".to_string(),
                    ));
                }
            },
            None => {
                return Err(LoggerInitializationError::LogPath(
                    "Could not get home directory".to_string(),
                ));
            }
        };
        let logfile_path = home_dir + "/.config/cmdstack/cmdstack.log";

        let logfile = FileAppender::builder().build(logfile_path)?;

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder().appender("logfile").build(LevelFilter::Info))
            .map_err(|_| LoggerInitializationError::CreatingLogConfig)?;

        log4rs::init_config(config).map_err(|_| LoggerInitializationError::InitializingLogger)?;
    }

    Ok(())
}

fn main() {
    match initialize_logger() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error initializing logger: {}", e);
            std::process::exit(1);
        }
    }

    // Configure inquire
    inquire::set_global_render_config(inquire::ui::RenderConfig {
        prompt: inquire::ui::StyleSheet::default().with_fg(inquire::ui::Color::LightBlue),
        ..Default::default()
    });

    // Parse command line arguments and execute the command
    let args = CmdStackArgs::parse();

    match args.command {
        Command::Add(add_args) => command::add_command::handle_add_command(add_args),
        Command::Update(update_args) => command::update_command::handle_update_command(update_args),
        Command::Delete(delete_args) => command::delete_command::handle_delete_command(delete_args),
        Command::Search(search_args) => {
            command::search_command::handle_search_commands(search_args)
        }
        Command::List(list_args) => command::list_commands::handle_list_commands(list_args),
        Command::Param(param_args) => param::handle_param_command(param_args),
        Command::Export(import_export_args) => {
            import_export::handle_export_command(import_export_args)
        }
        Command::Import(import_export_args) => {
            import_export::handle_import_command(import_export_args)
        }
    }
}
