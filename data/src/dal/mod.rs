pub mod sqlite;
pub mod sqlite_dal;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InsertCommandError {
    #[error("Failed to get the unix timestamp: {0}")]
    UnixTimestamp(#[from] std::time::SystemTimeError),
    #[error("Expected rows to be affected after insertion but none were affected")]
    NoRowsAffected,
    #[error("Failed to execute SQL query: {0}")]
    Query(#[from] sqlx::Error),
    #[error("Failed to build SQL query to insert: {0}")]
    QueryBuilder(#[from] sea_query::error::Error),
}

#[derive(Error, Debug)]
pub enum UpdateCommandError {
    #[error("Failed to get the unix timestamp: {0}")]
    UnixTimestamp(#[from] std::time::SystemTimeError),
    #[error("Expected rows to be affected after update but none were affected")]
    NoRowsAffected,
    #[error("Failed to execute SQL query: {0}")]
    Query(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum DeleteCommandError {
    #[error("Expected rows to be affected after deletion but none were affected")]
    NoRowsAffected,
    #[error("Failed to execute SQL query: {0}")]
    Query(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum SelectAllCommandsError {
    #[error("Failed to execute SQL query: {0}")]
    Query(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum SqlTxError {
    #[error("Failed to begin transaction: {0}")]
    TxBegin(#[source] sqlx::Error),

    #[error("Failed to commit transaction: {0}")]
    TxCommit(#[source] sqlx::Error),

    #[error("Failed to rollback transaction: {0}")]
    TxRollback(#[source] sqlx::Error),
}
