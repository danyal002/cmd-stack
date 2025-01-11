pub mod sqlite;
pub mod sqlite_dal;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqlQueryError {
    #[error("failed to add command")]
    AddCommand(#[source] sqlx::Error),

    #[error("failed to search for command")]
    SearchCommand(#[source] sqlx::Error),

    #[error("failed to update command last used property")]
    UpdateCommandLastUsed(#[source] sqlx::Error),

    #[error("failed to add parameter")]
    AddParam(#[source] sqlx::Error),
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
