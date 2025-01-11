pub mod sqlite;
pub mod sqlite_dal;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqlQueryError {
    #[error("Failed to select commands")]
    SelectCommand(#[source] sqlx::Error),

    #[error("Failed to insert a command")]
    InsertCommand(#[source] sqlx::Error),

    #[error("Failed to delete a command")]
    DeleteCommand(#[source] sqlx::Error),

    #[error("Failed to update a command")]
    UpdateCommand(#[source] sqlx::Error),

    #[error("Failed to get the unix timestamp")]
    UnixTimestamp(#[from] std::time::SystemTimeError),

    #[error("Expected rows to be affected by the operation but none were affected")]
    NoRowsAffected,
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
