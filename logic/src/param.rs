//! Handles all requests for commands
use data::{
    dal::Dal,
    models::{Command, InternalParameter},
};
use rand::Rng;
use rand_regex::Regex;
use thiserror::Error;

use data::dal::{sqlite::SqliteDatabase, SqlDal};

#[derive(Error, Debug)]
pub enum AddParamError {
    #[error("Invalid parameter")]
    InvalidParam,
    #[error("database creation error")]
    DbConnection(#[from] data::dal::sqlite::SQliteDatabaseConnectionError),
    #[error("unknown data store error")]
    Query,
}

#[tokio::main]
/// Handles the addition of parameters
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

    // Add the parameters to the database
    match dal.add_params(params).await {
        Ok(_) => {}
        Err(_) => return Err(AddParamError::Query),
    };

    Ok(())
}

#[derive(Error, Debug)]
pub enum GenerateParamError {
    #[error("database creation error")]
    DbConnection(#[from] data::dal::sqlite::SQliteDatabaseConnectionError),
    #[error("unknown data store error")]
    Query,
    #[error("invalid regex pattern")]
    InvalidRegexPattern(#[from] regex_syntax::Error),
    #[error("invalid Hir (high-level intermediate representation) for the regex pattern")]
    InvalidHir(#[from] rand_regex::Error),
}

#[tokio::main]
/// Handles the generation of parameters for a command
pub async fn handle_generate_param(command: Command) -> Result<String, GenerateParamError> {
    // Set up database connection
    let sqlite_db = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(GenerateParamError::DbConnection(e)),
    };
    let dal = SqlDal {
        sql: Box::new(sqlite_db),
    };

    // Get the parameters for the command from the database
    let params: Vec<InternalParameter> = match dal.get_params(command.id).await {
        Ok(p) => p,
        Err(_) => return Err(GenerateParamError::Query),
    };

    // If there are no parameters, return the command
    if params.is_empty() {
        return Ok(command.internal_command.command);
    }

    // Generate the parameters
    let mut rng = rand::thread_rng();

    let mut param_string = String::new();
    for param in params.iter() {
        let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
        let hir = parser.parse(&param.regex);
        if hir.is_err() {
            return Err(GenerateParamError::InvalidRegexPattern(hir.unwrap_err()));
        }

        let gen = match Regex::with_hir(hir.unwrap(), 100) {
            Ok(r) => r,
            Err(e) => return Err(GenerateParamError::InvalidHir(e)),
        };
        let param_value = (&mut rng)
            .sample_iter(&gen)
            .take(1)
            .collect::<Vec<String>>();

        param_string.push_str(&format!("{} {} ", param.symbol, param_value[0]));
    }

    Ok(command.internal_command.command + " " + &param_string)
}
