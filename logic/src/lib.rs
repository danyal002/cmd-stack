//! # Logic
//!
//! This crate handles the business logic of the application

pub mod command;
pub mod config;
pub mod import_export;
pub mod parameters;

use config::{Config, ConfigReadError};
use data::dal::{sqlite::SqliteDbConnectionError, sqlite_dal::SqliteDal};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LogicInitError {
    #[error("Failed to initalize the database connection: {0}")]
    Database(#[from] SqliteDbConnectionError),
    #[error("Failed to read from config file: {0}")]
    Config(#[from] ConfigReadError),
}

pub struct Logic {
    dal: SqliteDal,
    pub config: Config,
}

impl Logic {
    pub fn new(dal: SqliteDal) -> Result<Logic, LogicInitError> {
        Ok(Logic {
            dal,
            config: Config::read()?,
        })
    }

    pub fn try_default() -> Result<Logic, LogicInitError> {
        Ok(Self {
            dal: SqliteDal::new()?,
            config: Config::read()?,
        })
    }
}
