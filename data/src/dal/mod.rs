pub mod sqlite;

use sea_query::{Query, SqliteQueryBuilder};
use std::error::Error;
use sqlite::{SqlDal, CommandSchema};
use async_trait::async_trait;

#[async_trait]
pub trait Dal: Sync + Send {
    async fn add_command(&self, alias: String, command: String, tag: Option<String>, note: Option<String>) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
impl Dal for SqlDal {
    async fn add_command(&self, alias: String, command: String, tag: Option<String>, note: Option<String>) -> Result<(), Box<dyn Error>> {
        let query = Query::insert()
            .into_table(CommandSchema::Table)
            .columns([
                CommandSchema::Alias,
                CommandSchema::Command,
                CommandSchema::Tag,
                CommandSchema::Note,
            ])
            .values_panic([
                alias.into(),
                command.into(),
                tag.into(),
                note.into()
            ]).to_string(SqliteQueryBuilder);
        
        println!("Query: {:?}", query);

        // Execute the query using your database connection
        // sqlx::query(&query).execute(&sql.pool).await?;

        Ok(())
    }
}
