pub mod sqlite;

use async_trait::async_trait;
use sea_query::{Query, SqliteQueryBuilder};
use sqlite::SqliteDatabase;
use sqlx::sqlite::SqliteRow;
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
}

#[derive(Error, Debug)]
pub enum SqliteQueryError {
    #[error("failed to add command")]
    AddCommand(#[source] sqlx::Error),
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
}
