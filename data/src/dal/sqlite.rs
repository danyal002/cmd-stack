use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, SqliteQueryBuilder, Table};
use sqlx::sqlite::SqlitePoolOptions;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SQliteDatabaseConnectionError {
    #[error("Could not get the current directory")]
    CurDir(#[source] std::io::Error),
    #[error("Could not create the database file")]
    CreatingDatabase(#[source] std::io::Error),
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
    /// Creates a new connection to a SQLite database and initializes the tables
    /// if required
    pub async fn new() -> Result<Self, SQliteDatabaseConnectionError> {
        // Create a connection pool
        let cur_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                return Err(SQliteDatabaseConnectionError::CurDir(e));
            }
        };

        let mut db_path = PathBuf::from(cur_dir);
        db_path.pop();
        db_path.push("data/cmdstack_db.db");

        // Check if the database file exists
        if !Path::new(&db_path).exists() {
            // Create the database file
            match File::create(&db_path) {
                Ok(_) => {}
                Err(e) => {
                    return Err(SQliteDatabaseConnectionError::CreatingDatabase(e));
                }
            }
        }

        let pool = match SqlitePoolOptions::new()
            .connect(&format!("sqlite://{}", db_path.to_str().unwrap()))
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                return Err(SQliteDatabaseConnectionError::PoolInitialization(e));
            }
        };

        // Create the tables
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
            .col(ColumnDef::new(Parameter::Name).string().not_null())
            .col(ColumnDef::new(Parameter::Symbol).string().not_null())
            .col(ColumnDef::new(Parameter::DefaultValue).string())
            .col(ColumnDef::new(Parameter::Note).string())
            .foreign_key(
                ForeignKey::create()
                    .name("fk_69420")
                    .from(Parameter::Table, Parameter::Id)
                    .to(Command::Table, Command::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .build(SqliteQueryBuilder);

        match sqlx::query(&command_table_sql).execute(&pool).await {
            Ok(_) => {}
            Err(e) => {
                return Err(SQliteDatabaseConnectionError::Command(e));
            }
        }
        match sqlx::query(&parameter_table_sql).execute(&pool).await {
            Ok(_) => {}
            Err(e) => {
                return Err(SQliteDatabaseConnectionError::Parameter(e));
            }
        }

        Ok(Self { pool })
    }
}

/// Command Table Schema
#[derive(Iden)]
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

// Parameter Table Schema
#[derive(Iden)]
pub enum Parameter {
    Table,
    Id,
    CommandId,
    Name,
    Symbol,
    DefaultValue,
    Note,
}
