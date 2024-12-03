//! # Logic
//!
//! This crate handles the business logic of the application

pub mod command;
pub mod import_export;
pub mod param;

use data::dal::{sqlite::SQliteDatabaseConnectionError, sqlite_dal::SqliteDal, Dal, SqlQueryError};
use sqlx::sqlite::SqliteRow;
use sqlx::Sqlite;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DefaultLogicError {
    #[error("failed to initalize the database connection")]
    DbConnection(#[from] DatabaseConnectionError),

    #[error("unknown data store error")]
    Query(#[from] SqlQueryError),
}

pub struct Logic {
    db_connection: Box<dyn Dal<Row = SqliteRow, DB = Sqlite>>,
}

impl Logic {
    pub fn new(dal: Box<dyn Dal<Row = SqliteRow, DB = Sqlite>>) -> Logic {
        Logic { db_connection: dal }
    }

    pub fn try_default() -> Result<Logic, DefaultLogicError> {
        let dal = match SqliteDal::new() {
            Ok(dal) => dal,
            Err(e) => {
                return Err(DefaultLogicError::DbConnection(
                    DatabaseConnectionError::SqliteError(e),
                ))
            }
        };
    
        Ok(Logic::new(Box::new(dal)))
    }
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
