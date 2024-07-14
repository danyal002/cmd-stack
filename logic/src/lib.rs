//! # Logic
//!
//! This crate handles the business logic of the application

pub mod command;
pub mod import_export;
pub mod param;

use data::dal::{
    sqlite::{SQliteDatabaseConnectionError, SqliteDatabase},
    sqlite_dal::SqliteDal,
    SqlQueryError,
};
use std::sync::OnceLock;
use thiserror::Error;

static DB_CONNECTION: OnceLock<SqliteDal> = OnceLock::new();

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

pub async fn init_db_connection() -> Result<SqliteDal, SQliteDatabaseConnectionError> {
    let sqlite_db = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(e),
    };

    Ok(SqliteDal {
        sql: Box::new(sqlite_db),
    })
}

pub async fn get_db_connection() -> Result<&'static SqliteDal, DatabaseConnectionError> {
    if let Some(dal) = DB_CONNECTION.get() {
        Ok(dal)
    } else {
        // If it is not initialized
        let dal = init_db_connection().await?;
        match DB_CONNECTION.set(dal) {
            Ok(_) => {}
            Err(_) => return Err(DatabaseConnectionError::InitDBConnection),
        }

        match DB_CONNECTION.get() {
            Some(dal) => Ok(dal),
            None => Err(DatabaseConnectionError::NoneAfterInit),
        }
    }
}
