//! # Logic
//!
//! This crate handles the business logic of the application

use data::dal::{sqlite::{SqliteDatabase, SQliteDatabaseConnectionError}, sqlite_dal::SqliteDal};
use thiserror::Error;

pub mod command;
pub mod param;
pub mod import_export;

#[derive(Debug, Error)]
pub enum LogicInitError {
    #[error("database connection error")]
    DbConnection(#[from] SQliteDatabaseConnectionError),
}

pub struct Logic {
    dal: SqliteDal
}

impl Logic {
    #[tokio::main]
    pub async fn new() -> Result<Self, LogicInitError> {
        let sqlite_db = match SqliteDatabase::new().await {
            Ok(db) => db,
            Err(e) => return Err(LogicInitError::DbConnection(e)),
        };
        let dal = SqliteDal {
            sql: Box::new(sqlite_db),
        };

        Ok(Self { dal })
    } 
}

