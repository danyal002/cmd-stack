//! Handles all requests for commands
use data::{
    dal::Dal,
    models::InternalParameter,
};
use thiserror::Error;

use data::dal::{sqlite::SqliteDatabase, SqlDal};

#[derive(Error, Debug)]
pub enum AddParamError {
    #[error("Invalid command")]
    InvalidParam,
    #[error("database creation error")]
    DbConnection(#[from] data::dal::sqlite::SQliteDatabaseConnectionError),
    #[error("unknown data store error")]
    Query,
}

#[tokio::main]
/// Handles the addition of a command
pub async fn handle_add_param(params: Vec<InternalParameter>) -> Result<(), AddParamError> {
    for param in params.iter() {
        if param.symbol.trim().is_empty() || param.regex.trim().is_empty() {
            return Err(AddParamError::InvalidParam);
        }
    }

    // Set up database connection
    let sqlite_db = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(AddParamError::DbConnection(e)),
    };
    let dal = SqlDal {
        sql: Box::new(sqlite_db),
    };

    // Add the command to the database
    match dal
        .add_params(params)
        .await
    {
        Ok(_) => {}
        Err(_) => return Err(AddParamError::Query),
    };

    Ok(())
}
