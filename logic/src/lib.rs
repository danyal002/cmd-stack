//! # Logic
//!
//! This crate handles the business logic of the application

pub mod command;
pub mod import_export;
pub mod param;

use data::dal::{sqlite::SQliteDatabaseConnectionError, sqlite_dal::SqliteDal, SqlQueryError};
use thiserror::Error;

pub struct Logic {
    db_connection: SqliteDal,
}

impl Logic {
    pub fn new(dal: SqliteDal) -> Logic {
        Logic { db_connection: dal }
    }
}

pub fn new_logic() -> Result<Logic, SQliteDatabaseConnectionError> {
    let dal = SqliteDal::new()?;
    Ok(Logic::new(dal))
}

#[derive(Debug, Error)]
pub enum DefaultLogicError {
    #[error("failed to initalize the database connection")]
    DbConnection(#[from] DatabaseConnectionError),

    #[error("unknown data store error")]
    Query(#[from] SqlQueryError),
}

#[derive(Debug, Error)]
pub enum DatabaseConnectionError {
    #[error("Failed to create database")]
    SqliteError(#[from] SQliteDatabaseConnectionError),

    #[error("Failed to initialize DB_CONNECTION")]
    InitDBConnection,

    #[error("Got none after initializing db connection")]
    NoneAfterInit,
}
