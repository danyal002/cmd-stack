use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, SqliteQueryBuilder, Table};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::{env, fs};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SQliteDatabaseConnectionError {
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

    #[error("Could not create parameter table")]
    Parameter(#[source] sqlx::Error),
}

/// Represents a connection to a SQLite database
pub struct SqliteDatabase {
    pub pool: sqlx::SqlitePool,
}

impl SqliteDatabase {
    /// Creates a new connection to a SQLite database
    pub async fn new() -> Result<Self, SQliteDatabaseConnectionError> {
        let pool = Self::establish_db_connection().await?;

        Self::create_tables(&pool).await?;

        Ok(Self { pool })
    }

    /// Returns path to database
    fn get_db_path() -> Result<String, SQliteDatabaseConnectionError> {
        let top_level_directory = match env::var_os("CMD_STACK_DIRECTORY") {
            Some(path) => {
                path.to_string_lossy().into_owned()
            },
            None => {
                match dirs::config_dir() {
                    Some(dir) => match dir.to_str() {
                        Some(path) => path.to_string(),
                        None => {
                            return Err(SQliteDatabaseConnectionError::DbPath(
                                "Could not convert home directory to string".to_string(),
                            ));
                        }
                    },
                    None => {
                        return Err(SQliteDatabaseConnectionError::DbPath(
                            "Could not get config directory".to_string(),
                        ));
                    }
                }
            }
        };

        // We must create he directory to allow SQLite to create the database file
        let directory = top_level_directory + "/cmdstack/";
        match fs::create_dir_all(directory.clone()) {
            Ok(_) => {},
            Err(e) => {
                return Err(SQliteDatabaseConnectionError::CreatingDatabase(e));
            }
        }

        Ok(directory + "database.sqlite")
    }

    async fn establish_db_connection() -> Result<SqlitePool, SQliteDatabaseConnectionError> {
        let db_path = Self::get_db_path()?;

        let mut connect_options = match SqliteConnectOptions::from_str(&db_path) {
            Ok(options) => options,
            Err(e) => {
                return Err(SQliteDatabaseConnectionError::SqliteOptionsInitialization(
                    e,
                ))
            }
        };
        // Enable foreign keys and ensure database file is created if it does not exist
        connect_options = connect_options.foreign_keys(true).create_if_missing(true);

        match SqlitePool::connect_with(connect_options).await {
            Ok(pool) => Ok(pool),
            Err(e) => Err(SQliteDatabaseConnectionError::PoolInitialization(e)),
        }
    }

    async fn create_tables(pool: &SqlitePool) -> Result<(), SQliteDatabaseConnectionError> {
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

        let parameter_table_sql = Table::create()
            .table(Parameter::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Parameter::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Parameter::CommandId).integer().not_null())
            .col(ColumnDef::new(Parameter::Symbol).string().not_null())
            .col(ColumnDef::new(Parameter::Regex).string().not_null())
            .col(ColumnDef::new(Parameter::Note).string())
            .foreign_key(
                ForeignKey::create()
                    .name("fk_69420")
                    .from(Parameter::Table, Parameter::CommandId)
                    .to(Command::Table, Command::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .build(SqliteQueryBuilder);

        match sqlx::query(&command_table_sql).execute(pool).await {
            Ok(_) => {}
            Err(e) => {
                return Err(SQliteDatabaseConnectionError::Command(e));
            }
        }
        match sqlx::query(&parameter_table_sql).execute(pool).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SQliteDatabaseConnectionError::Parameter(e)),
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

#[derive(Iden)]
/// Parameter Table Schema
pub enum Parameter {
    Table,
    Id,
    CommandId,
    Symbol,
    Regex,
    Note,
}
