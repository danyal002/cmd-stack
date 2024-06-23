//! # Data
//!
//! This crate is responsible for accessing the local SQLite database

use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, SqliteQueryBuilder, Table};
use sqlx::sqlite::SqlitePoolOptions;
use std::error::Error;
use std::path::PathBuf;

pub async fn initialize_database() -> Result<(), Box<dyn Error>> {
    // Create a connection pool
    let mut db_path = PathBuf::from(std::env::current_dir()?);
    db_path.push("cmdstack_db.db");

    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite://{}", db_path.to_str().unwrap()))
        .await?;

    // Define a table
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
                .name("fk_1")
                .from(Parameter::Table, Parameter::Id)
                .to(Command::Table, Command::Id)
                .on_delete(ForeignKeyAction::Cascade),
        )
        .build(SqliteQueryBuilder);

    // Execute the SQL statement
    sqlx::query(&command_table_sql).execute(&pool).await?;
    sqlx::query(&parameter_table_sql).execute(&pool).await?;

    println!("Database initialized and table created.");

    Ok(())
}

/// Command Table
#[derive(Iden)]
enum Command {
    Table,
    Id,
    Alias,
    Command,
    Tag,
    Note,
    LastUsed,
}

// Parameter Table
#[derive(Iden)]
enum Parameter {
    Table,
    Id,
    CommandId,
    Name,
    Symbol,
    DefaultValue,
    Note,
}
