//! # Logic
//!
//! This crate handles the business logic of the application

pub mod command;
pub mod import_export;
pub mod param;

use data::dal::{sqlite::SqliteDbConnectionError, sqlite_dal::SqliteDal};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LogicInitError {
    #[error("Failed to initalize the database connection")]
    Database(#[from] SqliteDbConnectionError),
}

pub struct Logic {
    dal: SqliteDal,
}

impl Logic {
    pub fn new(dal: SqliteDal) -> Logic {
        Logic { dal }
    }

    pub fn try_default() -> Result<Logic, LogicInitError> {
        let dal = SqliteDal::new().map_err(LogicInitError::Database)?;
        Ok(Logic::new(dal))
    }
}
