//! Handles all requests for commands
use data::models::{Command, InternalCommand};
use data::dal::{Dal, sqlite::SqliteDatabase, sqlite_dal::SqliteDal, SqlQueryError};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AddCommandError {
    #[error("Invalid command")]
    InvalidCommand,
    #[error("database creation error")]
    DbConnection(#[from] data::dal::sqlite::SQliteDatabaseConnectionError),
    #[error("unknown data store error")]
    Query(#[from] SqlQueryError),
}

#[tokio::main]
/// Handles the addition of a command
pub async fn handle_add_command(command: InternalCommand) -> Result<(), AddCommandError> {
    if command.command.trim().is_empty() || command.alias.trim().is_empty() {
        return Err(AddCommandError::InvalidCommand);
    }

    // Set up database connection
    let sqlite_db = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(AddCommandError::DbConnection(e)),
    };
    let dal = SqliteDal {
        sql: Box::new(sqlite_db),
    };

    // Add the command to the database
    match dal.add_command(command).await {
        Ok(_) => {}
        Err(e) => return Err(AddCommandError::Query(e)),
    };

    Ok(())
}

#[derive(Debug)]
pub struct SearchCommandArgs {
    pub alias: Option<String>,
    pub command: Option<String>,
    pub tag: Option<String>,
}

#[derive(Error, Debug)]
pub enum SearchCommandError {
    #[error("database creation error")]
    DbConnection(#[from] data::dal::sqlite::SQliteDatabaseConnectionError),

    #[error("Error querying the database")]
    Query(#[from] SqlQueryError),
}

#[tokio::main]
/// Handles the search for a command
pub async fn handle_search_command(
    params: SearchCommandArgs,
) -> Result<Vec<Command>, SearchCommandError> {
    // Set up database connection
    let sqlite_db = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(SearchCommandError::DbConnection(e)),
    };
    let dal = SqliteDal {
        sql: Box::new(sqlite_db),
    };

    // Get all commands from the database
    let commands = match dal.get_all_commands(false, false).await {
        Ok(results) => results,
        Err(e) => return Err(SearchCommandError::Query(e)),
    };

    // Filter the commands based on the search parameters using fuzzy matching
    let matcher = SkimMatcherV2::default();
    let filtered_commands: Vec<Command> = commands
        .into_iter()
        .filter(|command| {
            // The minimum threshold for a match to be considered valid
            let min_threshold = 50; // TODO: Adjust this threshold

            let alias_match = match &params.alias {
                Some(a) => {
                    let res = matcher.fuzzy_match(&command.internal_command.alias, a);
                    res.is_some() && res.unwrap() > min_threshold
                }
                None => false,
            };

            let command_match = match &params.command {
                Some(c) => {
                    let res = matcher.fuzzy_match(&command.internal_command.command, c);
                    res.is_some() && res.unwrap() > min_threshold
                }
                None => false,
            };

            let tag_match = match &params.tag {
                Some(t) => match &command.internal_command.tag {
                    Some(tag) => {
                        let res = matcher.fuzzy_match(tag, t);
                        res.is_some() && res.unwrap() > min_threshold
                    }
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
) -> Result<Vec<Command>, SearchCommandError> {
    // Set up database connection
    let sqlite_db: SqliteDatabase = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(SearchCommandError::DbConnection(e)),
    };
    let dal = SqliteDal {
        sql: Box::new(sqlite_db),
    };

    // Get all commands from the database
    let commands = match dal.get_all_commands(order_by_use, favourite).await {
        Ok(results) => results,
        Err(e) => return Err(SearchCommandError::Query(e)),
    };

    Ok(commands)
}

#[derive(Error, Debug)]
pub enum UpdateCommandError {
    #[error("Invalid command")]
    InvalidCommand,

    #[error("database creation error")]
    DbConnection(data::dal::sqlite::SQliteDatabaseConnectionError),

    #[error("Error updating command last used property")]
    Query(#[from] SqlQueryError),
}

#[tokio::main]
/// Handles the updating of the last used property of a command
pub async fn handle_update_command_last_used_prop(
    command_id: i64,
) -> Result<(), UpdateCommandError> {
    // Set up database connection
    let sqlite_db = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(UpdateCommandError::DbConnection(e)),
    };
    let dal = SqliteDal {
        sql: Box::new(sqlite_db),
    };

    // Update the last used property of the command
    match dal.update_command_last_used_prop(command_id).await {
        Ok(_) => {}
        Err(e) => return Err(UpdateCommandError::Query(e)),
    };

    Ok(())
}

#[tokio::main]
/// Handles the updating of a command
pub async fn handle_update_command(
    command_id: i64,
    new_command_props: InternalCommand,
) -> Result<(), UpdateCommandError> {
    if new_command_props.command.trim().is_empty() || new_command_props.alias.trim().is_empty() {
        return Err(UpdateCommandError::InvalidCommand);
    }

    // Set up database connection
    let sqlite_db = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(UpdateCommandError::DbConnection(e)),
    };
    let dal = SqliteDal {
        sql: Box::new(sqlite_db),
    };

    // Update the command
    match dal.update_command(command_id, new_command_props).await {
        Ok(_) => {}
        Err(e) => return Err(UpdateCommandError::Query(e)),
    };

    Ok(())
}

#[derive(Error, Debug)]
pub enum DeleteCommandError {
    #[error("database creation error")]
    DbConnection(data::dal::sqlite::SQliteDatabaseConnectionError),

    #[error("Error deleting command")]
    Query(#[from] SqlQueryError),
}

#[tokio::main]
/// Handles deleting a command
pub async fn handle_delete_command(command_id: i64) -> Result<(), DeleteCommandError> {
    // Set up database connection
    let sqlite_db = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(DeleteCommandError::DbConnection(e)),
    };
    let dal = SqliteDal {
        sql: Box::new(sqlite_db),
    };

    // Delete the selected command
    match dal.delete_command(command_id).await {
        Ok(_) => {}
        Err(e) => return Err(DeleteCommandError::Query(e)),
    };

    Ok(())
}
