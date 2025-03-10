use sea_query::{ColumnDef, Iden, SqliteQueryBuilder, Table};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::fs;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqliteDbConnectionError {
    #[error("Could not get the database path: {0}")]
    DbPath(String),

    #[error("Could not create sqlite options: {0}")]
    SqliteOptionsInitialization(#[source] sqlx::Error),

    #[error("Could not connect to the file: {0}")]
    PoolInitialization(#[source] sqlx::Error),

    #[error("Could not create command table: {0}")]
    Command(#[source] sqlx::Error),
}

pub(crate) struct SqliteConnectionPool {
    pub(crate) pool: SqlitePool,
}

impl SqliteConnectionPool {
    pub async fn new(db_path: Option<String>) -> Result<Self, SqliteDbConnectionError> {
        let db_path = match db_path {
            Some(path) => path,
            None => Self::default_db_path()?,
        };

        let pool = Self::create_connection_pool(db_path).await?;

        Self::create_tables(&pool).await?;

        Ok(SqliteConnectionPool { pool })
    }

    /// Returns the default path to the database file. The default location for the database
    /// file is in the `cmdstack` directory which is located in the OS config directory.
    ///
    /// If the `cmdstack` directory does not exist, it is created
    fn default_db_path() -> Result<String, SqliteDbConnectionError> {
        let mut path = dirs::config_dir().ok_or_else(|| {
            SqliteDbConnectionError::DbPath("Could not get config directory".to_string())
        })?;
        path.push("cmdstack");

        // Create the config directory if it does not exist
        fs::create_dir_all(path.as_path()).map_err(|_| {
            SqliteDbConnectionError::DbPath(format!(
                "Could not create config directory: {:?}",
                path.to_str()
            ))
        })?;

        path.push("database.sqlite"); // Add the database file to the path
        path.to_str().map(|s| s.to_string()).ok_or_else(|| {
            SqliteDbConnectionError::DbPath("Could not generate the default db path".to_string())
        })
    }

    async fn create_connection_pool(
        db_path: String,
    ) -> Result<SqlitePool, SqliteDbConnectionError> {
        let connect_options = SqliteConnectOptions::from_str(&db_path)
            .map_err(SqliteDbConnectionError::SqliteOptionsInitialization)?
            .create_if_missing(true);

        SqlitePool::connect_with(connect_options)
            .await
            .map_err(SqliteDbConnectionError::PoolInitialization)
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
            .col(ColumnDef::new(Command::Command).string().not_null())
            .col(ColumnDef::new(Command::Tag).string())
            .col(ColumnDef::new(Command::Note).string())
            .col(ColumnDef::new(Command::LastUsed).integer().default(0))
            .col(ColumnDef::new(Command::Favourite).boolean().default(false))
            .build(SqliteQueryBuilder);

        sqlx::query(&command_table_sql)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(SqliteDbConnectionError::Command)
    }
}

#[derive(Iden)]
/// Command Table Schema
pub enum Command {
    Table,
    Id,
    Command,
    Tag,
    Note,
    LastUsed,
    Favourite,
}
