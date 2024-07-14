//! Handles all requests for commands
use data::{
    dal::Dal,
    models::{Command, InternalParameter, Parameter},
};
use rand::Rng;
use rand_regex::Regex;
use thiserror::Error;

use data::dal::SqlQueryError;

use crate::{get_db_connection, DatabaseConnectionError, DefaultLogicError};

#[derive(Error, Debug)]
pub enum AddParamError {
    #[error("invalid parameter")]
    InvalidParam,

    #[error("failed to initalize the database connection")]
    DbConnection(#[from] DatabaseConnectionError),

    #[error("error executing database query")]
    Query(#[from] SqlQueryError),
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
    let dal = get_db_connection().await?;

    // Add the parameters to the database
    match dal.add_params(params, None).await {
        Ok(_) => {}
        Err(e) => return Err(AddParamError::Query(e)),
    };

    Ok(())
}

#[derive(Error, Debug)]
pub enum GenerateParamError {
    #[error("failed to initalize the database connection")]
    DbConnection(#[from] DatabaseConnectionError),

    #[error("error executing database query")]
    Query(#[from] SqlQueryError),

    #[error("invalid regex pattern")]
    InvalidRegexPattern(#[from] regex_syntax::Error),

    #[error("invalid Hir (high-level intermediate representation) for the regex pattern")]
    InvalidHir(#[from] rand_regex::Error),
}

#[tokio::main]
/// Handles the generation of parameters for a command
pub async fn handle_generate_param(command: Command) -> Result<String, GenerateParamError> {
    // Set up database connection
    let dal = get_db_connection().await?;

    // Get the parameters for the command from the database
    let params: Vec<Parameter> = match dal.get_params(command.id, None).await {
        Ok(p) => p,
        Err(e) => return Err(GenerateParamError::Query(e)),
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
        let hir = match parser.parse(&param.internal_parameter.regex) {
            Ok(hir) => hir,
            Err(e) => {
                return Err(GenerateParamError::InvalidRegexPattern(e));
            }
        };

        let gen = match Regex::with_hir(hir, 100) {
            Ok(r) => r,
            Err(e) => return Err(GenerateParamError::InvalidHir(e)),
        };
        let param_value = (&mut rng)
            .sample_iter(&gen)
            .take(1)
            .collect::<Vec<String>>();

        param_string.push_str(&format!(
            "{} {} ",
            param.internal_parameter.symbol, param_value[0]
        ));
    }

    Ok(command.internal_command.command + " " + &param_string)
}

#[tokio::main]
pub async fn get_params(command_id: i64) -> Result<Vec<Parameter>, DefaultLogicError> {
    // Set up database connection
    let dal = get_db_connection().await?;

    // Get the parameters for the command from the database
    let params: Vec<Parameter> = match dal.get_params(command_id, None).await {
        Ok(p) => p,
        Err(e) => return Err(DefaultLogicError::Query(e)),
    };

    Ok(params)
}

#[tokio::main]
pub async fn update_param(
    param_id: i64,
    param: InternalParameter,
) -> Result<(), DefaultLogicError> {
    // Set up database connection
    let dal = get_db_connection().await?;

    // Update the parameter in the database
    match dal.update_param(param_id, param, None).await {
        Ok(_) => {}
        Err(e) => return Err(DefaultLogicError::Query(e)),
    };

    Ok(())
}

#[tokio::main]
pub async fn delete_param(param_id: i64) -> Result<(), DefaultLogicError> {
    // Set up database connection
    let dal = get_db_connection().await?;

    // Delete the parameter from the database
    match dal.delete_param(param_id, None).await {
        Ok(_) => {}
        Err(e) => return Err(DefaultLogicError::Query(e)),
    };

    Ok(())
}
