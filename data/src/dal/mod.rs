pub mod sqlite;

use async_trait::async_trait;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sqlite::SqliteDatabase;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

use crate::models::*;

/// Data Access Layer for Sqlite
pub struct SqlDal {
    pub sql: Box<SqliteDatabase>,
}

#[async_trait]
/// Data Access Layer trait that includes all the methods required to interact with the database
pub trait Dal: Sync + Send {
    type Row;

    /// Gets the current Unix timestamp
    async fn get_unix_timestamp() -> i64;

    /// Executes a query
    async fn execute(&self, query: &str) -> Result<(), sqlx::Error>;

    /// Queries the database and returns the rows
    async fn query(&self, query: &str) -> Result<Vec<Self::Row>, sqlx::Error>;

    /// Adds a command to the database
    async fn add_command(&self, command: InternalCommand) -> Result<(), SqliteQueryError>;

    /// Gets all commands from the database
    async fn get_all_commands(
        &self,
        order_by_use: bool,
        favourites_only: bool,
    ) -> Result<Vec<Command>, SqliteQueryError>;

    /// Updates the last used property of a command to the current time
    async fn update_command_last_used_prop(&self, command_id: u64) -> Result<(), SqliteQueryError>;

    /// Deletes a command from the database
    async fn delete_command(&self, command_id: u64) -> Result<(), SqliteQueryError>;

    // Update a command
    async fn update_command(
        &self,
        command_id: u64,
        new_command_props: InternalCommand,
    ) -> Result<(), SqliteQueryError>;

    /// Adds parameters to the database
    async fn add_params(&self, params: Vec<InternalParameter>) -> Result<(), SqliteQueryError>;

    /// Get parameters for a command
    async fn get_params(&self, command_id: u64) -> Result<Vec<Parameter>, SqliteQueryError>;

    /// Update a parameter
    async fn update_param(&self, param_id: u64, param: InternalParameter) -> Result<(), SqliteQueryError>;

    /// Delete a parameter
    async fn delete_param(&self, param_id: u64) -> Result<(), SqliteQueryError>;
}

#[derive(Error, Debug)]
pub enum SqliteQueryError {
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
impl Dal for SqlDal {
    type Row = SqliteRow;

    async fn get_unix_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64
    }

    async fn execute(&self, query: &str) -> Result<(), sqlx::Error> {
        sqlx::query(query).execute(&self.sql.pool).await.map(|_| ())
    }

    async fn query(&self, query: &str) -> Result<Vec<Self::Row>, sqlx::Error> {
        let rows = sqlx::query(query).fetch_all(&self.sql.pool).await?;
        Ok(rows)
    }

    async fn add_command(&self, command: InternalCommand) -> Result<(), SqliteQueryError> {
        let current_time = Self::get_unix_timestamp().await;

        let query = Query::insert()
            .into_table(sqlite::Command::Table)
            .columns([
                sqlite::Command::Alias,
                sqlite::Command::Command,
                sqlite::Command::Tag,
                sqlite::Command::Note,
                sqlite::Command::Favourite,
                sqlite::Command::LastUsed,
            ])
            .values_panic([
                command.alias.into(),
                command.command.into(),
                command.tag.into(),
                command.note.into(),
                command.favourite.into(),
                current_time.into(),
            ])
            .to_string(SqliteQueryBuilder);

        match self.execute(&query).await {
            Ok(_) => {}
            Err(e) => return Err(SqliteQueryError::AddCommand(e)),
        };

        Ok(())
    }

    async fn get_all_commands(
        &self,
        order_by_use: bool,
        favourites_only: bool,
    ) -> Result<Vec<Command>, SqliteQueryError> {
        let query = Query::select()
            .columns([
                sqlite::Command::Alias,
                sqlite::Command::Command,
                sqlite::Command::Tag,
                sqlite::Command::Note,
                sqlite::Command::Favourite,
                sqlite::Command::Id,
                sqlite::Command::LastUsed,
            ])
            .conditions(
                // Ternary operator that allows us to add expressions at runtime
                order_by_use,
                |q| {
                    q.order_by(sqlite::Command::LastUsed, sea_query::Order::Desc);
                },
                |_| {},
            )
            .conditions(
                favourites_only,
                |q| {
                    q.and_where(Expr::col(sqlite::Command::Favourite).is_in([true]));
                },
                |_| {},
            )
            .from(sqlite::Command::Table)
            .to_string(SqliteQueryBuilder);

        let rows = match self.query(&query).await {
            Ok(rows) => rows,
            Err(e) => return Err(SqliteQueryError::SearchCommand(e)),
        };

        let mut commands = Vec::new();
        for row in rows {
            commands.push(Command {
                internal_command: InternalCommand {
                    alias: row.get("alias"),
                    command: row.get("command"),
                    tag: row.get("tag"),
                    note: row.get("note"),
                    favourite: row.get("favourite"),
                },
                id: row.get::<i64, _>("id") as u64,
                last_used: row.get::<i64, _>("last_used") as u64,
            });
        }

        Ok(commands)
    }

    async fn update_command_last_used_prop(&self, command_id: u64) -> Result<(), SqliteQueryError> {
        let current_time = Self::get_unix_timestamp().await;

        let query = Query::update()
            .table(sqlite::Command::Table)
            .values([(sqlite::Command::LastUsed, current_time.into())])
            .and_where(Expr::col(sqlite::Command::Id).eq(command_id))
            .to_string(SqliteQueryBuilder);

        match self.execute(&query).await {
            Ok(_) => {}
            Err(e) => return Err(SqliteQueryError::UpdateCommandLastUsed(e)),
        };

        Ok(())
    }

    async fn delete_command(&self, command_id: u64) -> Result<(), SqliteQueryError> {
        let query = Query::delete()
            .from_table(sqlite::Command::Table)
            .and_where(Expr::col(sqlite::Command::Id).eq(command_id))
            .to_string(SqliteQueryBuilder);

        match self.execute(&query).await {
            Ok(_) => {}
            Err(e) => return Err(SqliteQueryError::UpdateCommandLastUsed(e)),
        };

        Ok(())
    }

    async fn update_command(
        &self,
        command_id: u64,
        new_command_props: InternalCommand,
    ) -> Result<(), SqliteQueryError> {
        let query = Query::update()
            .table(sqlite::Command::Table)
            .values([
                (sqlite::Command::Alias, new_command_props.alias.into()),
                (sqlite::Command::Command, new_command_props.command.into()),
                (sqlite::Command::Tag, new_command_props.tag.into()),
                (sqlite::Command::Note, new_command_props.note.into()),
                (
                    sqlite::Command::Favourite,
                    new_command_props.favourite.into(),
                ),
            ])
            .and_where(Expr::col(sqlite::Command::Id).eq(command_id))
            .to_string(SqliteQueryBuilder);

        match self.execute(&query).await {
            Ok(_) => {}
            Err(e) => return Err(SqliteQueryError::UpdateCommandLastUsed(e)),
        };

        Ok(())
    }

    async fn add_params(&self, params: Vec<InternalParameter>) -> Result<(), SqliteQueryError> {
        let mut builder = Query::insert()
            .into_table(sqlite::Parameter::Table)
            .columns([
                sqlite::Parameter::CommandId,
                sqlite::Parameter::Symbol,
                sqlite::Parameter::Regex,
                sqlite::Parameter::Note,
            ])
            .to_owned();

        for param in params.into_iter() {
            builder.values_panic(vec![
                param.command_id.into(),
                param.symbol.into(),
                param.regex.into(),
                param.note.into(),
            ]);
        }

        let query = builder.to_string(SqliteQueryBuilder);

        match self.execute(&query).await {
            Ok(_) => {}
            Err(e) => return Err(SqliteQueryError::AddParam(e)),
        }

        Ok(())
    }

    async fn get_params(
        &self,
        command_id: u64,
    ) -> Result<Vec<Parameter>, SqliteQueryError> {
        let query = Query::select()
            .columns([
                sqlite::Parameter::Id,
                sqlite::Parameter::Symbol,
                sqlite::Parameter::Regex,
                sqlite::Parameter::Note,
            ])
            .and_where(Expr::col(sqlite::Parameter::CommandId).eq(command_id))
            .from(sqlite::Parameter::Table)
            .to_string(SqliteQueryBuilder);

        let rows = match self.query(&query).await {
            Ok(rows) => rows,
            Err(e) => return Err(SqliteQueryError::SearchCommand(e)),
        };

        let mut params = Vec::new();
        for row in rows {
            params.push(Parameter {
                id: row.get::<i64, _>("id") as u64,
                internal_parameter: InternalParameter {
                    command_id: command_id,
                    symbol: row.get("symbol"),
                    regex: row.get("regex"),
                    note: row.get("note"),
                },
            });
        }

        Ok(params)
    }

    async fn update_param(
        &self,
        param_id: u64,
        param: InternalParameter,
    ) -> Result<(), SqliteQueryError> {
        let query = Query::update()
            .table(sqlite::Parameter::Table)
            .values([
                (sqlite::Parameter::Symbol, param.symbol.into()),
                (sqlite::Parameter::Regex, param.regex.into()),
                (sqlite::Parameter::Note, param.note.into()),
            ])
            .and_where(Expr::col(sqlite::Parameter::Id).eq(param_id))
            .to_string(SqliteQueryBuilder);

        match self.execute(&query).await {
            Ok(_) => {}
            Err(e) => return Err(SqliteQueryError::UpdateCommandLastUsed(e)),
        };

        Ok(())
    }

    async fn delete_param(&self, param_id: u64) -> Result<(), SqliteQueryError> {
        let query = Query::delete()
            .from_table(sqlite::Parameter::Table)
            .and_where(Expr::col(sqlite::Parameter::Id).eq(param_id))
            .to_string(SqliteQueryBuilder);

        match self.execute(&query).await {
            Ok(_) => {}
            Err(e) => return Err(SqliteQueryError::UpdateCommandLastUsed(e)),
        };

        Ok(())
    }
}
