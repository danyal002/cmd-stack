use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, SqliteQueryBuilder, Table};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SQliteDatabaseConnectionError {
    #[error("Could not get the current directory")]
    CurDir(#[source] std::io::Error),

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
    /// 
    /// Creates the database and initializes the tables if required
    pub async fn new() -> Result<Self, SQliteDatabaseConnectionError> {
        if !Self::db_file_exists() {
            Self::create_db()
        }

        let pool = Self::establish_db_connection().await?;

        Self::create_tables(&pool).await?;

        Ok(Self { pool })
    }
    
    /// Returns path to database
    /// 
    /// Path: $HOME/.config/cmdstack/database.sqlite
    fn get_db_path() -> String {
        let home_dir = dirs::home_dir().unwrap();
        home_dir.to_str().unwrap().to_string() + "/.config/cmdstack/database.sqlite"
    }

    /// Checks if the database file exists
    fn db_file_exists() -> bool {
        let db_path = Self::get_db_path();
        Path::new(&db_path).exists()
    }

    /// Creates database file
    fn create_db() {
        let db_path = Self::get_db_path();
        let db_dir = Path::new(&db_path).parent().unwrap();

        if !db_dir.exists() {
            fs::create_dir_all(db_dir).unwrap();
        }

        fs::File::create(db_path).unwrap();
    }

    async fn establish_db_connection() -> Result<SqlitePool, SQliteDatabaseConnectionError> {
        let mut connect_options = match SqliteConnectOptions::from_str(Self::get_db_path().as_str()) {
            Ok(options) => options,
            Err(e) => return Err(SQliteDatabaseConnectionError::SqliteOptionsInitialization(e))
        };
        connect_options = connect_options.foreign_keys(true);

        match SqlitePool::connect_with(connect_options).await {
            Ok(pool) => Ok(pool),
            Err(e) => Err(SQliteDatabaseConnectionError::PoolInitialization(e))
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
                    .from(Parameter::Table, Parameter::Id)
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
            Err(e) => Err(SQliteDatabaseConnectionError::Parameter(e))
        }
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
    Symbol,
    Regex,
    Note,
}
