use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, SqliteQueryBuilder, Table};
use sqlx::sqlite::SqlitePoolOptions;
use std::error::Error;
use std::path::PathBuf;


pub struct SqliteDatabase {
    pool: sqlx::SqlitePool,
}

pub struct SqlDal {
    pub sql: Box<SqliteDatabase>,
}

impl SqliteDatabase {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // Create a connection pool
        let mut db_path = PathBuf::from(std::env::current_dir()?);
        db_path.push("cmdstack_db.db");
    
        let pool = SqlitePoolOptions::new()
            .connect(&format!("sqlite://{}", db_path.to_str().unwrap()))
            .await?;
    
        // Create the tables 
        let command_table_sql = Table::create()
            .table(CommandSchema::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CommandSchema::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(CommandSchema::Alias).string().not_null())
            .col(ColumnDef::new(CommandSchema::Command).string().not_null())
            .col(ColumnDef::new(CommandSchema::Tag).string())
            .col(ColumnDef::new(CommandSchema::Note).string())
            .col(ColumnDef::new(CommandSchema::LastUsed).integer().default(0))
            .build(SqliteQueryBuilder);
    
        let parameter_table_sql = Table::create()
            .table(ParameterSchema::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ParameterSchema::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(ParameterSchema::CommandId).integer().not_null())
            .col(ColumnDef::new(ParameterSchema::Name).string().not_null())
            .col(ColumnDef::new(ParameterSchema::Symbol).string().not_null())
            .col(ColumnDef::new(ParameterSchema::DefaultValue).string())
            .col(ColumnDef::new(ParameterSchema::Note).string())
            .foreign_key(
                ForeignKey::create()
                    .name("fk_69420")
                    .from(ParameterSchema::Table, ParameterSchema::Id)
                    .to(CommandSchema::Table, CommandSchema::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .build(SqliteQueryBuilder);
    
        sqlx::query(&command_table_sql).execute(&pool).await?;
        sqlx::query(&parameter_table_sql).execute(&pool).await?;
    
        Ok(Self { pool })
    }
}

/// Command Table Schema
#[derive(Iden)]
pub enum CommandSchema {
    Table,
    Id,
    Alias,
    Command,
    Tag,
    Note,
    LastUsed,
}

// Parameter Table Schema
#[derive(Iden)]
pub enum ParameterSchema {
    Table,
    Id,
    CommandId,
    Name,
    Symbol,
    DefaultValue,
    Note,
}
