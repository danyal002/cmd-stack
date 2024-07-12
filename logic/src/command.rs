//! Handles all requests for commands
use data::dal::{Dal, SqlQueryError};
use data::models::{Command, InternalCommand};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use thiserror::Error;

use crate::{get_db_connection, DatabaseConnectionError, DefaultLogicError};

#[derive(Error, Debug)]
pub enum CommandLogicError {
    #[error("invalid command")]
    InvalidCommand,

    #[error("failed to initalize the database connection")]
    DbConnection(#[from] DatabaseConnectionError),

    #[error("error executing database query")]
    Query(#[from] SqlQueryError),
}

#[tokio::main]
/// Handles the addition of a command
pub async fn handle_add_command(command: InternalCommand) -> Result<(), CommandLogicError> {
    if command.command.trim().is_empty() || command.alias.trim().is_empty() {
        return Err(CommandLogicError::InvalidCommand);
    }

    // Set up database connection
    let dal = get_db_connection().await?;

    // Add the command to the database
    match dal.add_command(command, None).await {
        Ok(_) => {}
        Err(e) => return Err(CommandLogicError::Query(e)),
    };

    Ok(())
}

#[derive(Debug)]
pub struct SearchCommandArgs {
    pub alias: Option<String>,
    pub command: Option<String>,
    pub tag: Option<String>,
}

#[tokio::main]
/// Handles the search for a command
pub async fn handle_search_command(
    params: SearchCommandArgs,
) -> Result<Vec<Command>, DefaultLogicError> {
    // Set up database connection
    let dal = get_db_connection().await?;

    // Get all commands from the database
    let commands = match dal.get_all_commands(false, false, None).await {
        Ok(results) => results,
        Err(e) => return Err(DefaultLogicError::Query(e)),
    };

    // Filter the commands based on the search parameters using fuzzy matching
    let matcher = SkimMatcherV2::default();
    let filtered_commands: Vec<Command> = commands
        .into_iter()
        .filter(|command| {
            // The minimum threshold for a match to be considered valid
            let min_threshold = 50; // TODO: Adjust this threshold

            let alias_match = match &params.alias {
                Some(a) => match matcher.fuzzy_match(&command.internal_command.alias, a) {
                    Some(r) => r > min_threshold,
                    None => false,
                },
                None => false,
            };

            let command_match = match &params.command {
                Some(c) => match matcher.fuzzy_match(&command.internal_command.command, c) {
                    Some(r) => r > min_threshold,
                    None => false,
                },
                None => false,
            };

            let tag_match = match &params.tag {
                Some(t) => match &command.internal_command.tag {
                    Some(tag) => match matcher.fuzzy_match(tag, t) {
                        Some(r) => r > min_threshold,
                        None => false,
                    },
                    None => false,
                },
                None => false,
            };

            alias_match || command_match || tag_match
        })
        .collect();

    Ok(filtered_commands)
}

#[tokio::main]
/// Handles the listing of all commands
pub async fn handle_list_commands(
    order_by_use: bool,
    favourite: bool,
) -> Result<Vec<Command>, DefaultLogicError> {
    // Set up database connection
    let dal = get_db_connection().await?;

    // Get all commands from the database
    let commands = match dal.get_all_commands(order_by_use, favourite, None).await {
        Ok(results) => results,
        Err(e) => return Err(DefaultLogicError::Query(e)),
    };

    Ok(commands)
}

#[tokio::main]
/// Handles the updating of the last used property of a command
pub async fn handle_update_command_last_used_prop(
    command_id: i64,
) -> Result<(), CommandLogicError> {
    // Set up database connection
    let dal = get_db_connection().await?;

    // Update the last used property of the command
    match dal.update_command_last_used_prop(command_id, None).await {
        Ok(_) => {}
        Err(e) => return Err(CommandLogicError::Query(e)),
    };

    Ok(())
}

#[tokio::main]
/// Handles the updating of a command
pub async fn handle_update_command(
    command_id: i64,
    new_command_props: InternalCommand,
) -> Result<(), CommandLogicError> {
    if new_command_props.command.trim().is_empty() || new_command_props.alias.trim().is_empty() {
        return Err(CommandLogicError::InvalidCommand);
    }

    // Set up database connection
    let dal = get_db_connection().await?;

    // Update the command
    match dal
        .update_command(command_id, new_command_props, None)
        .await
    {
        Ok(_) => {}
        Err(e) => return Err(CommandLogicError::Query(e)),
    };

    Ok(())
}

#[tokio::main]
/// Handles deleting a command
pub async fn handle_delete_command(command_id: i64) -> Result<(), DefaultLogicError> {
    // Set up database connection
    let dal = get_db_connection().await?;

    // Delete the selected command
    match dal.delete_command(command_id, None).await {
        Ok(_) => {}
        Err(e) => return Err(DefaultLogicError::Query(e)),
    };

    Ok(())
}
