pub mod sqlite;
pub mod sqlite_dal;

use crate::models::*;
use async_trait::async_trait;
use sqlx::Transaction;
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

#[async_trait]
/// Data Access Layer trait that includes all the methods required to interact with the database
pub trait Dal: Sync + Send {
    type Row: sqlx::Row;
    type DB: sqlx::Database;

    /// Begin transaction
    async fn begin(&self) -> Result<Transaction<'_, Self::DB>, SqlTxError>;

    /// Commit transaction
    async fn commit(&self, tx: Transaction<'_, Self::DB>) -> Result<(), SqlTxError>;

    /// Rollback transaction
    async fn rollback(&self, tx: Transaction<'_, Self::DB>) -> Result<(), SqlTxError>;

    /// Gets the current Unix timestamp
    async fn get_unix_timestamp(&self) -> i64;

    /// Executes an insert
    async fn execute_insert(
        &self,
        query: &str,
        tx: Option<&mut Transaction<'_, Self::DB>>,
    ) -> Result<i64, sqlx::Error>;

    /// Executes a query (returns nothing)
    async fn execute(
        &self,
        query: &str,
        tx: Option<&mut Transaction<'_, Self::DB>>,
    ) -> Result<(), sqlx::Error>;

    /// Query the database (returns rows)
    async fn query(
        &self,
        query: &str,
        tx: Option<&mut Transaction<'_, Self::DB>>,
    ) -> Result<Vec<Self::Row>, sqlx::Error>;

    /// Adds a command to the database
    async fn add_command(
        &self,
        command: InternalCommand,
        tx: Option<&mut Transaction<'_, Self::DB>>,
    ) -> Result<i64, SqlQueryError>;

    /// Gets all commands from the database
    async fn get_all_commands(
        &self,
        order_by_use: bool,
        favourites_only: bool,
        tx: Option<&mut Transaction<'_, Self::DB>>,
    ) -> Result<Vec<Command>, SqlQueryError>;

    /// Updates the last used property of a command to the current time
    async fn update_command_last_used_prop(
        &self,
        command_id: i64,
        tx: Option<&mut Transaction<'_, Self::DB>>,
    ) -> Result<(), SqlQueryError>;

    /// Deletes a command from the database
    async fn delete_command(
        &self,
        command_id: i64,
        tx: Option<&mut Transaction<'_, Self::DB>>,
    ) -> Result<(), SqlQueryError>;

    /// Update a command
    async fn update_command(
        &self,
        command_id: i64,
        new_command_props: InternalCommand,
        tx: Option<&mut Transaction<'_, Self::DB>>,
    ) -> Result<(), SqlQueryError>;
}
