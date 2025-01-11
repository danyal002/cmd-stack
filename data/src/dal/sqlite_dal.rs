use super::sqlite::{SqliteConnectionPool, SqliteDbConnectionError};
use super::SqlQueryError;
use super::{sqlite, SqlTxError};
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Sqlite, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::*;

/// Data Access Layer for Sqlite
pub struct SqliteDal {
    pub(crate) sql: sqlite::SqliteConnectionPool,
}

impl SqliteDal {
    #[tokio::main]
    pub async fn new() -> Result<SqliteDal, SqliteDbConnectionError> {
        let sqlite_db = SqliteConnectionPool::new(None).await?;
        Ok(SqliteDal { sql: sqlite_db })
    }

    #[tokio::main]
    pub async fn new_with_directory(
        directory: String,
    ) -> Result<SqliteDal, SqliteDbConnectionError> {
        let sqlite_db = SqliteConnectionPool::new(Some(directory)).await?;
        Ok(SqliteDal { sql: sqlite_db })
    }

    pub async fn begin(&self) -> Result<Transaction<'_, Sqlite>, SqlTxError> {
        match self.sql.pool.begin().await {
            Ok(tx) => Ok(tx),
            Err(e) => Err(SqlTxError::TxBegin(e)),
        }
    }

    pub async fn rollback(&self, tx: Transaction<'_, Sqlite>) -> Result<(), SqlTxError> {
        match tx.rollback().await {
            Ok(_) => Ok(()),
            Err(e) => Err(SqlTxError::TxRollback(e)),
        }
    }

    pub async fn commit(&self, tx: Transaction<'_, Sqlite>) -> Result<(), SqlTxError> {
        match tx.commit().await {
            Ok(_) => Ok(()),
            Err(e) => Err(SqlTxError::TxCommit(e)),
        }
    }

    fn get_unix_timestamp(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64
    }

    async fn execute_insert(
        &self,
        query: &str,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<i64, sqlx::Error> {
        let query_result;
        if let Some(trans) = tx {
            query_result = sqlx::query(query).execute(&mut **trans).await?;
        } else {
            query_result = sqlx::query(query).execute(&self.sql.pool).await?;
        }
        Ok(query_result.last_insert_rowid())
    }

    async fn execute(
        &self,
        query: &str,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<(), sqlx::Error> {
        if let Some(trans) = tx {
            sqlx::query(query).execute(&mut **trans).await.map(|_| ())
        } else {
            sqlx::query(query).execute(&self.sql.pool).await.map(|_| ())
        }
    }

    async fn query(
        &self,
        query: &str,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<Vec<SqliteRow>, sqlx::Error> {
        let rows;
        if let Some(trans) = tx {
            rows = sqlx::query(query).fetch_all(&mut **trans).await?;
        } else {
            rows = sqlx::query(query).fetch_all(&self.sql.pool).await?;
        }
        Ok(rows)
    }

    pub async fn add_command(
        &self,
        command: InternalCommand,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<i64, SqlQueryError> {
        let current_time = self.get_unix_timestamp();

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

        let inserted_row_id = match self.execute_insert(&query, tx).await {
            Ok(id) => id,
            Err(e) => return Err(SqlQueryError::AddCommand(e)),
        };

        Ok(inserted_row_id)
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

        let rows = match self.query(&query, tx).await {
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
        let current_time = self.get_unix_timestamp();

        let query = Query::update()
            .table(sqlite::Command::Table)
            .values([(sqlite::Command::LastUsed, current_time.into())])
            .and_where(Expr::col(sqlite::Command::Id).eq(command_id))
            .to_string(SqliteQueryBuilder);

        match self.execute(&query, tx).await {
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

        match self.execute(&query, tx).await {
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

        match self.execute(&query, tx).await {
            Ok(_) => {}
            Err(e) => return Err(SqlQueryError::UpdateCommandLastUsed(e)),
        };

        Ok(())
    }
}
