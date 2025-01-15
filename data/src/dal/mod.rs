pub mod sqlite;
pub mod sqlite_dal;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InsertCommandError {
    #[error("Failed to get the unix timestamp")]
    UnixTimestamp(#[from] std::time::SystemTimeError),
    #[error("Expected rows to be affected by the operation but none were affected")]
    NoRowsAffected,
    #[error("Failed to execute SQL query")]
    Query(#[from] sqlx::Error),
    #[error("Failed to build SQL query to insert")]
    QueryBuilder(#[from] sea_query::error::Error),
}

#[derive(Error, Debug)]
pub enum UpdateCommandError {
    #[error("Failed to get the unix timestamp")]
    UnixTimestamp(#[from] std::time::SystemTimeError),
    #[error("Expected rows to be affected by the operation but none were affected")]
    NoRowsAffected,
    #[error("Failed to execute SQL query")]
    Query(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum DeleteCommandError {
    #[error("Expected rows to be affected by the operation but none were affected")]
    NoRowsAffected,
    #[error("Failed to execute SQL query")]
    Query(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum SelectAllCommandsError {
    #[error("Failed to execute SQL query")]
    Query(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum SqlTxError {
    #[error("Failed to begin transaction")]
    TxBegin(#[source] sqlx::Error),

    #[error("Failed to commit transaction")]
    TxCommit(#[source] sqlx::Error),

    #[error("Failed to rollback transaction")]
    TxRollback(#[source] sqlx::Error),
}
