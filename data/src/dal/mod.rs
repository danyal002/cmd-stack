pub mod sqlite;
pub mod sqlite_dal;

use async_trait::async_trait;
use thiserror::Error;
use crate::models::*;

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

#[async_trait]
/// Data Access Layer trait that includes all the methods required to interact with the database
pub trait Dal: Sync + Send {
    type Row;

    /// Gets the current Unix timestamp
    async fn get_unix_timestamp() -> i64;

    /// Executes an insert
    async fn execute_insert(&self, query: &str) -> Result<i64, sqlx::Error>;

    /// Executes a query
    async fn execute(&self, query: &str) -> Result<(), sqlx::Error>;

    /// Queries the database and returns the rows
    async fn query(&self, query: &str) -> Result<Vec<Self::Row>, sqlx::Error>;

    /// Adds a command to the database
    async fn add_command(&self, command: InternalCommand) -> Result<i64, SqlQueryError>;

    /// Gets all commands from the database
    async fn get_all_commands(
        &self,
        order_by_use: bool,
        favourites_only: bool,
    ) -> Result<Vec<Command>, SqlQueryError>;

    /// Updates the last used property of a command to the current time
    async fn update_command_last_used_prop(&self, command_id: i64) -> Result<(), SqlQueryError>;

    /// Deletes a command from the database
    async fn delete_command(&self, command_id: i64) -> Result<(), SqlQueryError>;

    /// Update a command
    async fn update_command(
        &self,
        command_id: i64,
        new_command_props: InternalCommand,
    ) -> Result<(), SqlQueryError>;

    /// Adds parameters to the database
    async fn add_params(&self, params: Vec<InternalParameter>) -> Result<(), SqlQueryError>;

    /// Get parameters for a command
    async fn get_params(&self, command_id: i64) -> Result<Vec<Parameter>, SqlQueryError>;

    /// Update a parameter
    async fn update_param(
        &self,
        param_id: i64,
        param: InternalParameter,
    ) -> Result<(), SqlQueryError>;

    /// Delete a parameter
    async fn delete_param(&self, param_id: i64) -> Result<(), SqlQueryError>;

    /// Get all parameters
    async fn get_all_internal_parameters(&self) -> Result<Vec<InternalParameter>, SqlQueryError>;
}
