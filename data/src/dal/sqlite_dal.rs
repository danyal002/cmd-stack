use super::sqlite::{SqliteConnectionPool, SqliteDbConnectionError};
use super::{sqlite, DeleteCommandError, SqlTxError, UpdateCommandError};
use super::{InsertCommandError, SelectAllCommandsError};
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sqlx::sqlite::{SqliteQueryResult, SqliteRow};
use sqlx::{Row, Sqlite, Transaction};
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use crate::models::*;

/// The Data Access Layer
///
/// The interface of this struct allows for the use of transactions
pub struct SqliteDal {
    pub(crate) sqlite_conn: sqlite::SqliteConnectionPool,
}

impl SqliteDal {
    /// Connects to the database at the default location
    #[tokio::main]
    pub async fn new() -> Result<SqliteDal, SqliteDbConnectionError> {
        let sqlite_db = SqliteConnectionPool::new(None).await?;
        Ok(SqliteDal {
            sqlite_conn: sqlite_db,
        })
    }

    /// Connects to the database at the provided file path
    #[tokio::main]
    pub async fn new_with_custom_path(
        custom_path: String,
    ) -> Result<SqliteDal, SqliteDbConnectionError> {
        let sqlite_db = SqliteConnectionPool::new(Some(custom_path)).await?;
        Ok(SqliteDal {
            sqlite_conn: sqlite_db,
        })
    }

    pub async fn begin(&self) -> Result<Transaction<'_, Sqlite>, SqlTxError> {
        self.sqlite_conn
            .pool
            .begin()
            .await
            .map_err(SqlTxError::TxBegin)
    }

    /// Takes ownership of the given transaction object
    pub async fn rollback(&self, tx: Transaction<'_, Sqlite>) -> Result<(), SqlTxError> {
        tx.rollback().await.map_err(SqlTxError::TxRollback)
    }

    /// Takes ownership of the given transaction object
    pub async fn commit(&self, tx: Transaction<'_, Sqlite>) -> Result<(), SqlTxError> {
        tx.commit().await.map_err(SqlTxError::TxCommit)
    }

    /// Returns the current unix timestamp in seconds
    fn get_unix_timestamp(&self) -> Result<i64, SystemTimeError> {
        let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?;
        Ok(duration_since_epoch.as_secs() as i64)
    }

    async fn execute_query(
        &self,
        query: &str,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        let result = if let Some(transaction) = tx {
            sqlx::query(query).execute(&mut **transaction).await?
        } else {
            sqlx::query(query).execute(&self.sqlite_conn.pool).await?
        };
        Ok(result)
    }

    async fn read_rows(
        &self,
        query: &str,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<Vec<SqliteRow>, sqlx::Error> {
        let rows = if let Some(transaction) = tx {
            sqlx::query(query).fetch_all(&mut **transaction).await?
        } else {
            sqlx::query(query).fetch_all(&self.sqlite_conn.pool).await?
        };
        Ok(rows)
    }

    pub async fn get_all_commands(
        &self,
        order_by_use: bool,
        favourites_only: bool,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<Vec<Command>, SelectAllCommandsError> {
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

        let rows = self
            .read_rows(&query, tx)
            .await
            .map_err(SelectAllCommandsError::Query)?;

        let commands: Vec<Command> = rows
            .into_iter()
            .map(|row| Command {
                internal_command: InternalCommand {
                    alias: row.get("alias"),
                    command: row.get("command"),
                    tag: row.get("tag"),
                    note: row.get("note"),
                    favourite: row.get("favourite"),
                },
                id: row.get("id"),
                last_used: row.get("last_used"),
            })
            .collect();

        Ok(commands)
    }

    /// Inserts a command and returns the ID of the inserted command
    pub async fn insert_command(
        &self,
        command: InternalCommand,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<i64, InsertCommandError> {
        let current_time = self.get_unix_timestamp()?;

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

        let result = self
            .execute_query(&query, tx)
            .await
            .map_err(InsertCommandError::Query)?;

        if result.rows_affected() == 0 {
            return Err(InsertCommandError::NoRowsAffected);
        }

        Ok(result.last_insert_rowid())
    }

    pub async fn insert_mulitple_commands(
        &self,
        commands: Vec<InternalCommand>,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<u64, InsertCommandError> {
        if commands.is_empty() {
            return Err(InsertCommandError::NoRowsAffected);
        }
        let current_time = self.get_unix_timestamp()?;

        let mut builder = Query::insert()
            .into_table(sqlite::Command::Table)
            .columns([
                sqlite::Command::Alias,
                sqlite::Command::Command,
                sqlite::Command::Tag,
                sqlite::Command::Note,
                sqlite::Command::Favourite,
                sqlite::Command::LastUsed,
            ])
            .to_owned();

        for command in commands {
            builder
                .values([
                    command.alias.into(),
                    command.command.into(),
                    command.tag.into(),
                    command.note.into(),
                    command.favourite.into(),
                    current_time.into(),
                ])
                .map_err(InsertCommandError::QueryBuilder)?;
        }

        let query = builder.to_string(SqliteQueryBuilder);
        let result = self
            .execute_query(&query, tx)
            .await
            .map_err(InsertCommandError::Query)?;

        if result.rows_affected() == 0 {
            return Err(InsertCommandError::NoRowsAffected);
        }

        Ok(result.rows_affected())
    }

    pub async fn update_command_last_used_property(
        &self,
        command_id: i64,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<(), UpdateCommandError> {
        let current_time = self.get_unix_timestamp()?;

        let query = Query::update()
            .table(sqlite::Command::Table)
            .values([(sqlite::Command::LastUsed, current_time.into())])
            .and_where(Expr::col(sqlite::Command::Id).eq(command_id))
            .to_string(SqliteQueryBuilder);

        let result = self
            .execute_query(&query, tx)
            .await
            .map_err(UpdateCommandError::Query)?;

        if result.rows_affected() == 0 {
            return Err(UpdateCommandError::NoRowsAffected);
        }

        Ok(())
    }

    pub async fn delete_command(
        &self,
        command_id: i64,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<(), DeleteCommandError> {
        let query = Query::delete()
            .from_table(sqlite::Command::Table)
            .and_where(Expr::col(sqlite::Command::Id).eq(command_id))
            .to_string(SqliteQueryBuilder);

        let result = self
            .execute_query(&query, tx)
            .await
            .map_err(DeleteCommandError::Query)?;

        if result.rows_affected() == 0 {
            return Err(DeleteCommandError::NoRowsAffected);
        }

        Ok(())
    }

    pub async fn update_command(
        &self,
        command_id: i64,
        new_command_props: InternalCommand,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<(), UpdateCommandError> {
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

        let result = self
            .execute_query(&query, tx)
            .await
            .map_err(UpdateCommandError::Query)?;

        if result.rows_affected() == 0 {
            return Err(UpdateCommandError::NoRowsAffected);
        }

        Ok(())
    }
}
