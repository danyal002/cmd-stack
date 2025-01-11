use super::sqlite::{SqliteConnectionPool, SqliteDbConnectionError};
use super::SqlQueryError;
use super::{sqlite, SqlTxError};
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sqlx::sqlite::{SqliteQueryResult, SqliteRow};
use sqlx::{Row, Sqlite, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::*;

/// The Data Access Layer
///
/// The interface of this struct allows for the use of transactions
pub struct SqliteDal {
    pub(crate) sqlite_conn: sqlite::SqliteConnectionPool,
}

impl SqliteDal {
    #[tokio::main]
    pub async fn new() -> Result<SqliteDal, SqliteDbConnectionError> {
        let sqlite_db = SqliteConnectionPool::new(None).await?;
        Ok(SqliteDal {
            sqlite_conn: sqlite_db,
        })
    }

    #[tokio::main]
    pub async fn new_with_directory(
        directory: String,
    ) -> Result<SqliteDal, SqliteDbConnectionError> {
        let sqlite_db = SqliteConnectionPool::new(Some(directory)).await?;
        Ok(SqliteDal {
            sqlite_conn: sqlite_db,
        })
    }

    pub async fn begin(&self) -> Result<Transaction<'_, Sqlite>, SqlTxError> {
        self.sqlite_conn
            .pool
            .begin()
            .await
            .map_err(|e| SqlTxError::TxBegin(e))
    }

    /// Takes ownership of the given transaction object
    pub async fn rollback(&self, tx: Transaction<'_, Sqlite>) -> Result<(), SqlTxError> {
        tx.rollback().await.map_err(|e| SqlTxError::TxRollback(e))
    }

    /// Takes ownership of the given transaction object
    pub async fn commit(&self, tx: Transaction<'_, Sqlite>) -> Result<(), SqlTxError> {
        tx.commit().await.map_err(|e| SqlTxError::TxCommit(e))
    }

    /// Returns the current unix timestamp in seconds
    fn get_unix_timestamp(&self) -> Result<i64, SqlQueryError> {
        let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?;
        Ok(duration_since_epoch.as_secs() as i64)
    }

    async fn execute_query(
        &self,
        query: &str,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        let result = match tx {
            Some(transaction) => sqlx::query(query).execute(&mut **transaction).await?,
            None => sqlx::query(query).execute(&self.sqlite_conn.pool).await?,
        };
        Ok(result)
    }

    async fn read_rows(
        &self,
        query: &str,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<Vec<SqliteRow>, sqlx::Error> {
        let rows = match tx {
            Some(transaction) => sqlx::query(query).fetch_all(&mut **transaction).await?,
            None => sqlx::query(query).fetch_all(&self.sqlite_conn.pool).await?,
        };
        Ok(rows)
    }

    pub async fn add_command(
        &self,
        command: InternalCommand,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<i64, SqlQueryError> {
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

        let result = match self.execute_query(&query, tx).await {
            Ok(id) => id,
            Err(e) => return Err(SqlQueryError::AddCommand(e)),
        };

        Ok(result.last_insert_rowid())
    }

    pub async fn get_all_commands(
        &self,
        order_by_use: bool,
        favourites_only: bool,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<Vec<Command>, SqlQueryError> {
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

        let rows = match self.read_rows(&query, tx).await {
            Ok(rows) => rows,
            Err(e) => return Err(SqlQueryError::SearchCommand(e)),
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
                id: row.get::<i64, _>("id"),
                last_used: row.get::<i64, _>("last_used"),
            });
        }

        Ok(commands)
    }

    pub async fn update_command_last_used_prop(
        &self,
        command_id: i64,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<(), SqlQueryError> {
        let current_time = self.get_unix_timestamp()?;

        let query = Query::update()
            .table(sqlite::Command::Table)
            .values([(sqlite::Command::LastUsed, current_time.into())])
            .and_where(Expr::col(sqlite::Command::Id).eq(command_id))
            .to_string(SqliteQueryBuilder);

        match self.execute_query(&query, tx).await {
            Ok(_) => {}
            Err(e) => return Err(SqlQueryError::UpdateCommandLastUsed(e)),
        };

        Ok(())
    }

    pub async fn delete_command(
        &self,
        command_id: i64,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<(), SqlQueryError> {
        let query = Query::delete()
            .from_table(sqlite::Command::Table)
            .and_where(Expr::col(sqlite::Command::Id).eq(command_id))
            .to_string(SqliteQueryBuilder);

        match self.execute_query(&query, tx).await {
            Ok(_) => {}
            Err(e) => return Err(SqlQueryError::UpdateCommandLastUsed(e)),
        };

        Ok(())
    }

    pub async fn update_command(
        &self,
        command_id: i64,
        new_command_props: InternalCommand,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<(), SqlQueryError> {
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

        match self.execute_query(&query, tx).await {
            Ok(_) => {}
            Err(e) => return Err(SqlQueryError::UpdateCommandLastUsed(e)),
        };

        Ok(())
    }
}
