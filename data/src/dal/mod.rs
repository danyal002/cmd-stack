pub mod sqlite;
pub mod sqlite_dal;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqlQueryError {
    #[error("failed to select commands")]
    SelectCommand(#[source] sqlx::Error),

    #[error("failed to insert a command")]
    InsertCommand(#[source] sqlx::Error),

    #[error("failed to delete a command")]
    DeleteCommand(#[source] sqlx::Error),

    #[error("failed to update a command")]
    UpdateCommand(#[source] sqlx::Error),

    #[error("failed to get the unix timestamp")]
    UnixTimestamp(#[from] std::time::SystemTimeError),

    #[error("expected rows to be affected by the operation but none were affected")]
    NoRowsAffected,
}

#[derive(Error, Debug)]
pub enum SqlTxError {
    #[error("failed to begin transaction")]
    TxBegin(#[source] sqlx::Error),

    #[error("failed to commit transaction")]
    TxCommit(#[source] sqlx::Error),

    #[error("failed to rollback transaction")]
    TxRollback(#[source] sqlx::Error),
}
