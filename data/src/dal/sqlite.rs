use sea_query::{ColumnDef, Iden, SqliteQueryBuilder, Table};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::fs;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqliteDbConnectionError {
    #[error("Could not get the database path")]
    DbPath(String),

    #[error("Could not create the database file")]
    CreatingDatabase(#[source] std::io::Error),

    #[error("Could not create sqlite options")]
    SqliteOptionsInitialization(#[source] sqlx::Error),

    #[error("Could not connect to the file")]
    PoolInitialization(#[source] sqlx::Error),

    #[error("Could not create command table")]
    Command(#[source] sqlx::Error),
}

/// Represents a connection to a SQLite database
pub(crate) struct SqliteConnectionPool {
    pub(crate) pool: sqlx::SqlitePool,
}

impl SqliteConnectionPool {
    pub async fn new(db_path: Option<String>) -> Result<Self, SqliteDbConnectionError> {
        let db_path = if let Some(d) = db_path {
            d
        } else {
            Self::default_db_path()?
        };

        let pool = Self::create_connection_pool(db_path).await?;

        Self::create_tables(&pool).await?;

        Ok(SqliteConnectionPool { pool })
    }

    fn default_db_path() -> Result<String, SqliteDbConnectionError> {
        let top_level_directory = match dirs::config_dir() {
            Some(dir) => match dir.to_str() {
                Some(path) => path.to_string(),
                None => {
                    return Err(SqliteDbConnectionError::DbPath(
                        "Could not convert config directory to string".to_string(),
                    ));
                }
            },
            None => {
                return Err(SqliteDbConnectionError::DbPath(
                    "Could not get config directory".to_string(),
                ));
            }
        };

        // We must create the directory to allow SQLite to create the database file
        let directory = top_level_directory + "/cmdstack/";
        match fs::create_dir_all(directory.clone()) {
            Ok(_) => {}
            Err(e) => {
                return Err(SqliteDbConnectionError::CreatingDatabase(e));
            }
        }

        Ok(directory + "database.sqlite")
    }

    async fn create_connection_pool(
        db_path: String,
    ) -> Result<SqlitePool, SqliteDbConnectionError> {
        let mut connect_options = match SqliteConnectOptions::from_str(&db_path) {
            Ok(options) => options,
            Err(e) => return Err(SqliteDbConnectionError::SqliteOptionsInitialization(e)),
        };

        // Ensure database file is created if it does not exist
        connect_options = connect_options.create_if_missing(true);

        match SqlitePool::connect_with(connect_options).await {
            Ok(pool) => Ok(pool),
            Err(e) => Err(SqliteDbConnectionError::PoolInitialization(e)),
        }
    }

    /// Initializes the tables in the Sqlite database if they do not exist
    async fn create_tables(pool: &SqlitePool) -> Result<(), SqliteDbConnectionError> {
        let command_table_sql = Table::create()
            .table(Command::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Command::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Command::Alias).string().not_null())
            .col(ColumnDef::new(Command::Command).string().not_null())
            .col(ColumnDef::new(Command::Tag).string())
            .col(ColumnDef::new(Command::Note).string())
            .col(ColumnDef::new(Command::LastUsed).integer().default(0))
            .col(ColumnDef::new(Command::Favourite).boolean().default(false))
            .build(SqliteQueryBuilder);

        match sqlx::query(&command_table_sql).execute(pool).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SqliteDbConnectionError::Command(e)),
        }
    }
}

#[derive(Iden)]
/// Command Table Schema
pub enum Command {
    Table,
    Id,
    Alias,
    Command,
    Tag,
    Note,
    LastUsed,
    Favourite,
}
