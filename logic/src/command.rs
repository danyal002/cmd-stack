//! Handles all requests for commands
use data::dal::Dal;
use thiserror::Error;

use data::dal::sqlite::SqlDal;
use data::dal::sqlite::SqliteDatabase;

#[derive(Debug)]
pub struct AddCommandParams {
    pub command: String,
    pub alias: String,
    pub tag: Option<String>,
    pub note: Option<String>,
}

#[derive(Error, Debug)]
pub enum AddCommandError {
    #[error("unknown data store error")]
    Unknown,
}

pub async fn handle_add_command(params: AddCommandParams) -> Result<(), AddCommandError> {
    let sqlite_db = SqliteDatabase::new().await.unwrap();
    let dal = SqlDal {
        sql: Box::new(sqlite_db),
    };

    dal.add_command(params.alias, params.command, params.tag, params.note).await.unwrap();

    Ok(())
}