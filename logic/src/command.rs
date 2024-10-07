//! Handles all requests for commands
use data::dal::Dal;
use data::dal::SqlQueryError;
use data::models::{Command, InternalCommand};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use thiserror::Error;

use crate::{DatabaseConnectionError, DefaultLogicError, Logic};

#[derive(Error, Debug)]
pub enum CommandLogicError {
    #[error("invalid command")]
    InvalidCommand,

    #[error("failed to initalize the database connection")]
    DbConnection(#[from] DatabaseConnectionError),

    #[error("error executing database query")]
    Query(#[from] SqlQueryError),
}

#[derive(Debug)]
pub struct SearchCommandArgs {
    pub alias: Option<String>,
    pub command: Option<String>,
    pub tag: Option<String>,
}

impl Logic {
    #[tokio::main]
    /// Handles the addition of a command
    pub async fn handle_add_command(
        &self,
        command: InternalCommand,
    ) -> Result<(), CommandLogicError> {
        if command.command.trim().is_empty() || command.alias.trim().is_empty() {
            return Err(CommandLogicError::InvalidCommand);
        }

        // Add the command to the database
        match self.db_connection.add_command(command, None).await {
            Ok(_) => {}
            Err(e) => return Err(CommandLogicError::Query(e)),
        };

        Ok(())
    }

    #[tokio::main]
    /// Handles the search for a command
    pub async fn handle_search_command(
        &self,
        params: SearchCommandArgs,
    ) -> Result<Vec<Command>, DefaultLogicError> {
        // Get all commands from the database
        let commands = match self
            .db_connection
            .get_all_commands(false, false, None)
            .await
        {
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
        &self,
        order_by_use: bool,
        favourite: bool,
    ) -> Result<Vec<Command>, DefaultLogicError> {
        // Get all commands from the database
        let commands = match self
            .db_connection
            .get_all_commands(order_by_use, favourite, None)
            .await
        {
            Ok(results) => results,
            Err(e) => return Err(DefaultLogicError::Query(e)),
        };

        Ok(commands)
    }

    #[tokio::main]
    /// Handles the updating of the last used property of a command
    pub async fn handle_update_command_last_used_prop(
        &self,
        command_id: i64,
    ) -> Result<(), CommandLogicError> {
        // Update the last used property of the command
        match self
            .db_connection
            .update_command_last_used_prop(command_id, None)
            .await
        {
            Ok(_) => {}
            Err(e) => return Err(CommandLogicError::Query(e)),
        };

        Ok(())
    }

    #[tokio::main]
    /// Handles the updating of a command
    pub async fn handle_update_command(
        &self,
        command_id: i64,
        new_command_props: InternalCommand,
    ) -> Result<(), CommandLogicError> {
        if new_command_props.command.trim().is_empty() || new_command_props.alias.trim().is_empty()
        {
            return Err(CommandLogicError::InvalidCommand);
        }

        // Update the command
        match self
            .db_connection
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
    pub async fn handle_delete_command(&self, command_id: i64) -> Result<(), DefaultLogicError> {
        // Delete the selected command
        match self.db_connection.delete_command(command_id, None).await {
            Ok(_) => {}
            Err(e) => return Err(DefaultLogicError::Query(e)),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use data::dal::sqlite_dal::SqliteDal;
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_handle_add_command_success() {
        let tmp_dir_result = TempDir::new();
        assert!(tmp_dir_result.is_ok());

        let path = tmp_dir_result
            .unwrap()
            .path()
            .to_string_lossy()
            .into_owned();
        let dal = SqliteDal::new_with_directory(path);
        assert!(dal.is_ok());
        let logic = Logic::new(dal.unwrap());

        let command = InternalCommand {
            command: "test_command".to_string(),
            alias: "test_alias".to_string(),
            tag: None,
            note: None,
            favourite: false,
        };

        let result = logic.handle_add_command(command.clone());
        assert!(result.is_ok());

        let list_commands_result = logic.handle_list_commands(false, false);
        assert!(list_commands_result.is_ok());
        let commands = list_commands_result.unwrap();
        assert!(commands.len() == 1);
        assert!(commands.first().unwrap().internal_command == command);
    }

    #[test]
    fn test_handle_update_command_success() {
        let tmp_dir_result = TempDir::new();
        assert!(tmp_dir_result.is_ok());

        let path = tmp_dir_result
            .unwrap()
            .path()
            .to_string_lossy()
            .into_owned();
        let dal = SqliteDal::new_with_directory(path);
        assert!(dal.is_ok());
        let logic = Logic::new(dal.unwrap());

        let command = InternalCommand {
            command: "test_command".to_string(),
            alias: "test_alias".to_string(),
            tag: None,
            note: None,
            favourite: false,
        };

        let result = logic.handle_add_command(command.clone());
        assert!(result.is_ok());

        let list_commands_result = logic.handle_list_commands(false, false);
        assert!(list_commands_result.is_ok());
        let commands = list_commands_result.unwrap();
        assert!(commands.len() == 1);
        assert!(commands.first().unwrap().internal_command == command);

        let new_command = InternalCommand {
            command: "new_test_command".to_string(),
            alias: "new_test_alias".to_string(),
            tag: Some("green".to_string()),
            note: Some("new note".to_string()),
            favourite: true,
        };

        let update_command_result =
            logic.handle_update_command(commands.first().unwrap().id, new_command.clone());
        assert!(update_command_result.is_ok());

        let list_commands_result = logic.handle_list_commands(false, false);
        assert!(list_commands_result.is_ok());
        let commands = list_commands_result.unwrap();
        assert!(commands.len() == 1);

        assert!(commands.first().unwrap().internal_command == new_command);
    }
}
