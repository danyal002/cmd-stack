//! Handles all requests for commands
use data::{dal::Dal, models::{InternalCommand, Command}};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use thiserror::Error;

use data::dal::{sqlite::SqliteDatabase, SqlDal};

#[derive(Debug)]
pub struct AddCommandParams {
    pub command: String,
    pub alias: String,
    pub tag: Option<String>,
    pub note: Option<String>,
    pub favourite: bool,
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
        Err(e) => return Err(AddCommandError::DbConnection(e)),
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
            favourite: params.favourite,
        })
        .await
    {
        Ok(_) => {}
        Err(_) => return Err(AddCommandError::Query),
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
    Query,
}

#[tokio::main]
pub async fn handle_search_command(
    params: SearchCommandArgs,
) -> Result<Vec<Command>, SearchCommandError> {
    let sqlite_db = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(SearchCommandError::DbConnection(e)),
    };
    let dal = SqlDal {
        sql: Box::new(sqlite_db),
    };

    let commands = match dal.get_all_commands(false, false).await {
        Ok(results) => results,
        Err(_) => return Err(SearchCommandError::Query),
    };

    let matcher = SkimMatcherV2::default();
    let filtered_commands: Vec<Command> = commands
        .into_iter()
        .filter(|command| {
            let alias_match = match &params.alias {
                Some(a) => matcher.fuzzy_match(&command.internal_command.alias, a).is_some(),
                None => false,
            };

            let command_match = match &params.command {
                Some(c) => matcher.fuzzy_match(&command.internal_command.command, c).is_some(),
                None => false,
            };

            let tag_match = match &params.tag {
                Some(t) => match &command.internal_command.tag {
                    Some(tag) => matcher.fuzzy_match(tag, t).is_some(),
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
pub async fn handle_list_commands(
    order_by_use: bool,
    favourite: bool,
) -> Result<Vec<Command>, SearchCommandError> {
    let sqlite_db: SqliteDatabase = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(SearchCommandError::DbConnection(e)),
    };
    let dal = SqlDal {
        sql: Box::new(sqlite_db),
    };

    let commands = match dal.get_all_commands(order_by_use, favourite).await {
        Ok(results) => results,
        Err(_) => return Err(SearchCommandError::Query),
    };

    Ok(commands)
}

#[derive(Error, Debug)]
pub enum UpdateCommandError {
    #[error("database creation error")]
    DbConnection(data::dal::sqlite::SQliteDatabaseConnectionError),

    #[error("Error updating command last used property")]
    Query,
}

#[tokio::main]
pub async fn handle_update_command_last_used_prop(command_id: u64) -> Result<(), UpdateCommandError> {
    let sqlite_db = match SqliteDatabase::new().await {
        Ok(db) => db,
        Err(e) => return Err(UpdateCommandError::DbConnection(e)),
    };
    let dal = SqlDal {
        sql: Box::new(sqlite_db),
    };

    match dal.update_command_last_used_prop(command_id).await {
        Ok(_) => {}
        Err(_) => return Err(UpdateCommandError::Query),
    };

    Ok(())
}