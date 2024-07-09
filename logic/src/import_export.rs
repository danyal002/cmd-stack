use data::dal::{sqlite::SqliteDatabase, Dal, SqlDal, SqliteQueryError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImportExportError {
    #[error("database connection error")]
    DbConnection(#[from] data::dal::sqlite::SQliteDatabaseConnectionError),

    #[error("database query error")]
    DbQuery(#[from] SqliteQueryError),
}

#[tokio::main]
/// Returns a JSON string containing all commands and parameters
pub async fn create_export_json(_destination_file_path: String) -> Result<(), ImportExportError> {
    // Set up database connection
    let sqlite_db = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(ImportExportError::DbConnection(e)),
    };
    let dal = SqlDal {
        sql: Box::new(sqlite_db),
    };

    // Get all commands and parameters
    let commands = dal.get_all_commands(false, false).await?;
    let parameters = dal.get_all_internal_parameters().await?;

    // Create JSON string
    let json_string = serde_json::json!({
        "commands": commands,
        "parameters": parameters,
    });

    println!("{}", json_string);

    Ok(())
}
