pub mod sqlite;

use async_trait::async_trait;
use sea_query::{Query, SqliteQueryBuilder};
use sqlite::SqliteDatabase;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use thiserror::Error;

use crate::models::InternalCommand;

pub struct SqlDal {
    pub sql: Box<SqliteDatabase>,
}

#[async_trait]
pub trait Dal: Sync + Send {
    type Row;

    async fn execute(&self, query: &str) -> Result<(), sqlx::Error>;
    async fn query(&self, query: &str) -> Result<Vec<Self::Row>, sqlx::Error>;
    async fn add_command(&self, command: InternalCommand) -> Result<(), SqliteQueryError>;
    async fn get_all_commands(&self) -> Result<Vec<InternalCommand>, SqliteQueryError>;
}

#[derive(Error, Debug)]
pub enum SqliteQueryError {
    #[error("failed to add command")]
    AddCommand(#[source] sqlx::Error),
    #[error("failed to search for command")]
    SearchCommand(#[source] sqlx::Error),
}

#[async_trait]
impl Dal for SqlDal {
    type Row = SqliteRow;

    async fn execute(&self, query: &str) -> Result<(), sqlx::Error> {
        sqlx::query(query).execute(&self.sql.pool).await.map(|_| ())
    }

    async fn query(&self, query: &str) -> Result<Vec<Self::Row>, sqlx::Error> {
        let rows = sqlx::query(query).fetch_all(&self.sql.pool).await?;
        Ok(rows)
    }

    async fn add_command(&self, command: InternalCommand) -> Result<(), SqliteQueryError> {
        let query = Query::insert()
            .into_table(sqlite::Command::Table)
            .columns([
                sqlite::Command::Alias,
                sqlite::Command::Command,
                sqlite::Command::Tag,
                sqlite::Command::Note,
            ])
            .values_panic([
                command.alias.into(),
                command.command.into(),
                command.tag.into(),
                command.note.into(),
            ])
            .to_string(SqliteQueryBuilder);

        match self.execute(&query).await {
            Ok(_) => {}
            Err(e) => return Err(SqliteQueryError::AddCommand(e)),
        };

        Ok(())
    }

    async fn get_all_commands(&self) -> Result<Vec<InternalCommand>, SqliteQueryError> {
        let query = Query::select()
            .columns([
                sqlite::Command::Alias,
                sqlite::Command::Command,
                sqlite::Command::Tag,
                sqlite::Command::Note,
            ])
            .from(sqlite::Command::Table)
            .to_string(SqliteQueryBuilder);

        let rows = match self.query(&query).await {
            Ok(rows) => rows,
            Err(e) => return Err(SqliteQueryError::SearchCommand(e)),
        };

        let mut commands = Vec::new();
        for row in rows {
            commands.push(InternalCommand {
                alias: row.get(0),
                command: row.get(1),
                tag: row.get(2),
                note: row.get(3),
            });
        }

        Ok(commands)
    }
}
