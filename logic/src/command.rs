//! Handles all requests for commands
use data::{dal::Dal, models::InternalCommand};
use thiserror::Error;

use data::dal::{sqlite::SqliteDatabase, SqlDal};

#[derive(Debug)]
pub struct AddCommandParams {
    pub command: String,
    pub alias: String,
    pub tag: Option<String>,
    pub note: Option<String>,
}

#[derive(Error, Debug)]
pub enum AddCommandError {
    #[error("database creation error")]
    DbConnection(#[from] data::dal::sqlite::SQliteDatabaseConnectionError),
    #[error("unknown data store error")]
    Query,
}

#[tokio::main]
pub async fn handle_add_command(params: AddCommandParams) -> Result<(), AddCommandError> {
    let sqlite_db = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => {
            return Err(AddCommandError::DbConnection(e))
        },
    };
    let dal = SqlDal {
        sql: Box::new(sqlite_db),
    };

    match dal
        .add_command(InternalCommand {
            alias: params.alias,
            command: params.command,
            tag: params.tag,
            note: params.note,
        })
        .await
    {
        Ok(_) => {}
        Err(_) => return Err(AddCommandError::Query),
    };

    Ok(())
}
