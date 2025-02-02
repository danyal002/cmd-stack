use data::dal::{InsertCommandError, SelectAllCommandsError};
use data::models::{Command, InternalCommand};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use thiserror::Error;

use crate::param::{ParameterError, ParameterHandler, SerializableParameter};
use crate::Logic;

#[derive(Error, Debug)]
pub enum AddCommandError {
    #[error("Invalid user input")]
    InvalidInput,
    #[error("Failed to add command")]
    Database(#[from] InsertCommandError),
    #[error("Failed validate parameters")]
    Parameter(#[from] ParameterError),
}

#[derive(Error, Debug)]
pub enum UpdateCommandError {
    #[error("Invalid user input")]
    InvalidInput,
    #[error("Failed to update command")]
    Database(#[from] data::dal::UpdateCommandError),
    #[error("Failed validate parameters")]
    Parameter(#[from] ParameterError),
}

#[derive(Error, Debug)]
pub enum SearchCommandError {
    #[error("Invalid user input")]
    InvalidInput,
    #[error("Failed to select commands")]
    Database(#[from] SelectAllCommandsError),
}

#[derive(Error, Debug)]
pub enum ListCommandError {
    #[error("Invalid user input")]
    InvalidInput,
    #[error("Failed to list commands")]
    Database(#[from] SelectAllCommandsError),
}

#[derive(Error, Debug)]
pub enum DeleteCommandError {
    #[error("Invalid user input")]
    InvalidInput,
    #[error("Failed to delete command")]
    Database(#[from] data::dal::DeleteCommandError),
}

#[derive(Debug)]
pub struct SearchCommandArgs {
    pub command: Option<String>,
    pub tag: Option<String>,
    pub order_by_recently_used: bool,
    pub favourites_only: bool,
}

impl Logic {
    #[tokio::main]
    /// Handles the addition of a command
    pub async fn add_command(&self, command: InternalCommand) -> Result<(), AddCommandError> {
        if command.command.trim().is_empty() {
            return Err(AddCommandError::InvalidInput);
        }

        ParameterHandler::default().validate_parameters(command.command.clone())?;

        self.dal.insert_command(command).await?;

        Ok(())
    }

    #[tokio::main]
    /// Handles the search for a command
    pub async fn search_command(
        &self,
        params: SearchCommandArgs,
    ) -> Result<Vec<Command>, SearchCommandError> {
        // Get all commands from the database
        let commands = self
            .dal
            .get_all_commands(params.order_by_recently_used, params.favourites_only)
            .await?;

        // Filter the commands based on the search parameters using fuzzy matching
        let matcher = SkimMatcherV2::default();
        let filtered_commands: Vec<Command> = commands
            .into_iter()
            .filter(|command| {
                // The minimum threshold for a match to be considered valid
                let min_threshold = 50; // TODO: Adjust this threshold

                // All commands if there is no filter
                if params.command.is_none() && params.tag.is_none() {
                    return true;
                }

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

                command_match || tag_match
            })
            .collect();

        Ok(filtered_commands)
    }

    #[tokio::main]
    /// Handles the listing of all commands
    pub async fn list_commands(
        &self,
        order_by_use: bool,
        favourite: bool,
    ) -> Result<Vec<Command>, ListCommandError> {
        // Get all commands from the database
        let commands = self.dal.get_all_commands(order_by_use, favourite).await?;

        Ok(commands)
    }

    #[tokio::main]
    /// Handles the updating of the last used property of a command
    pub async fn update_command_last_used_prop(
        &self,
        command_id: i64,
    ) -> Result<(), UpdateCommandError> {
        self.dal
            .update_command_last_used_property(command_id)
            .await?;
        Ok(())
    }

    #[tokio::main]
    /// Handles the updating of a command
    pub async fn update_command(
        &self,
        command_id: i64,
        new_command_props: InternalCommand,
    ) -> Result<(), UpdateCommandError> {
        if new_command_props.command.trim().is_empty() {
            return Err(UpdateCommandError::InvalidInput);
        }

        ParameterHandler::default().validate_parameters(new_command_props.command.clone())?;

        self.dal
            .update_command(command_id, new_command_props)
            .await?;

        Ok(())
    }

    #[tokio::main]
    /// Handles deleting a command
    pub async fn delete_command(&self, command_id: i64) -> Result<(), DeleteCommandError> {
        self.dal.delete_command(command_id).await?;

        Ok(())
    }

    #[tokio::main]
    /// Handles the generation of parameters for a command
    pub async fn generate_parameters(
        &self,
        command: String,
    ) -> Result<(String, Vec<String>), ParameterError> {
        ParameterHandler::default().replace_parameters(command)
    }

    #[tokio::main]
    /// Handles the parsing of parameters for a command
    pub async fn parse_parameters(
        &self,
        command: String,
    ) -> Result<(Vec<String>, Vec<SerializableParameter>), ParameterError> {
        ParameterHandler::default().parse_parameters(command)
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

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
        let dal = SqliteDal::new_with_custom_path(path);
        assert!(dal.is_ok());
        let logic = Logic::new(dal.unwrap());

        let command = InternalCommand {
            command: "test_command".to_string(),
            tag: None,
            note: None,
            favourite: false,
        };

        let result = logic.add_command(command.clone());
        assert!(result.is_ok());

        let list_commands_result = logic.list_commands(false, false);
        assert!(list_commands_result.is_ok());
        let commands = list_commands_result.unwrap();
        assert!(commands.len() == 1);
        assert!(commands.first().unwrap().internal_command == command);
    }

    #[test]
    fn test_handle_invalid_command() {
        let tmp_dir_result = TempDir::new();
        assert!(tmp_dir_result.is_ok());

        let path = tmp_dir_result
            .unwrap()
            .path()
            .to_string_lossy()
            .into_owned();
        let dal = SqliteDal::new_with_custom_path(path);
        assert!(dal.is_ok());
        let logic = Logic::new(dal.unwrap());

        let mut invalid_command = InternalCommand {
            command: "@{bad}".to_string(),
            tag: None,
            note: None,
            favourite: false,
        };

        let result = logic.add_command(invalid_command.clone());
        assert!(result.is_err());

        // Now a valid command
        invalid_command.command = "asdf".to_string();

        let result = logic.add_command(invalid_command.clone());
        assert!(result.is_ok());

        // Now an invalid command
        invalid_command.command = "@{what}".to_string();

        let list_commands_result = logic.list_commands(false, false);
        let commands = list_commands_result.unwrap();
        assert!(commands.len() == 1);

        let result = logic.update_command(commands[0].id, invalid_command.clone());
        assert!(result.is_err());

        // Now a valid command
        invalid_command.command = "@{int}".to_string();

        let result = logic.update_command(commands[0].id, invalid_command.clone());
        assert!(result.is_ok());
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
        let dal = SqliteDal::new_with_custom_path(path);
        assert!(dal.is_ok());
        let logic = Logic::new(dal.unwrap());

        let command = InternalCommand {
            command: "test_command".to_string(),
            tag: None,
            note: None,
            favourite: false,
        };

        let result = logic.add_command(command.clone());
        assert!(result.is_ok());

        let list_commands_result = logic.list_commands(false, false);
        assert!(list_commands_result.is_ok());
        let commands = list_commands_result.unwrap();
        assert!(commands.len() == 1);
        assert!(commands.first().unwrap().internal_command == command);

        let new_command = InternalCommand {
            command: "new_test_command".to_string(),
            tag: Some("green".to_string()),
            note: Some("new note".to_string()),
            favourite: true,
        };

        let update_command_result =
            logic.update_command(commands.first().unwrap().id, new_command.clone());
        assert!(update_command_result.is_ok());

        let list_commands_result = logic.list_commands(false, false);
        assert!(list_commands_result.is_ok());
        let commands = list_commands_result.unwrap();
        assert!(commands.len() == 1);

        assert!(commands.first().unwrap().internal_command == new_command);
    }

    #[test]
    fn test_handle_search_command_success() {
        let tmp_dir_result = TempDir::new();
        assert!(tmp_dir_result.is_ok());

        let path = tmp_dir_result
            .unwrap()
            .path()
            .to_string_lossy()
            .into_owned();
        let dal = SqliteDal::new_with_custom_path(path);
        assert!(dal.is_ok());
        let logic = Logic::new(dal.unwrap());

        let command = InternalCommand {
            command: "abcd".to_string(),
            tag: Some("green".to_string()),
            note: None,
            favourite: false,
        };
        let result = logic.add_command(command.clone());
        assert!(result.is_ok());

        let command = InternalCommand {
            command: "abce".to_string(),
            tag: Some("greet".to_string()),
            note: None,
            favourite: false,
        };
        let result = logic.add_command(command.clone());
        assert!(result.is_ok());

        // search by tag starts with gree
        let search_command_result = logic.search_command(SearchCommandArgs {
            command: None,
            tag: Some("gree".to_string()),
            order_by_recently_used: false,
            favourites_only: false,
        });
        assert!(search_command_result.is_ok());
        let commands = search_command_result.unwrap();
        assert!(commands.len() == 2);

        // search by tag starts with green
        let search_command_result = logic.search_command(SearchCommandArgs {
            command: None,
            tag: Some("green".to_string()),
            order_by_recently_used: false,
            favourites_only: false,
        });
        assert!(search_command_result.is_ok());
        let commands = search_command_result.unwrap();
        assert!(commands.len() == 1);

        // search by command starts with abc
        let search_command_result = logic.search_command(SearchCommandArgs {
            command: Some("abc".to_string()),
            tag: None,
            order_by_recently_used: false,
            favourites_only: false,
        });
        assert!(search_command_result.is_ok());
        let commands = search_command_result.unwrap();
        assert!(commands.len() == 2);

        // search by command starts with abcd
        let search_command_result = logic.search_command(SearchCommandArgs {
            command: Some("abcd".to_string()),
            tag: None,
            order_by_recently_used: false,
            favourites_only: false,
        });
        assert!(search_command_result.is_ok());
        let commands = search_command_result.unwrap();
        assert!(commands.len() == 1);

        // No filter should return all commands
        let search_command_result = logic.search_command(SearchCommandArgs {
            command: None,
            tag: None,
            order_by_recently_used: false,
            favourites_only: false,
        });
        assert!(search_command_result.is_ok());
        let commands = search_command_result.unwrap();
        assert!(commands.len() == 2);
    }

    #[test]
    fn test_handle_delete_command_success() {
        let tmp_dir_result = TempDir::new();
        assert!(tmp_dir_result.is_ok());

        let path = tmp_dir_result
            .unwrap()
            .path()
            .to_string_lossy()
            .into_owned();
        let dal = SqliteDal::new_with_custom_path(path);
        assert!(dal.is_ok());
        let logic = Logic::new(dal.unwrap());

        let command = InternalCommand {
            command: "test_command".to_string(),
            tag: None,
            note: None,
            favourite: false,
        };

        let result = logic.add_command(command.clone());
        assert!(result.is_ok());

        let list_commands_result = logic.list_commands(false, false);
        assert!(list_commands_result.is_ok());
        let commands = list_commands_result.unwrap();
        assert!(commands.len() == 1);
        let command_id = commands.first().unwrap().id;

        let delete_command_result = logic.delete_command(command_id);
        assert!(delete_command_result.is_ok());

        let list_commands_result = logic.list_commands(false, false);
        assert!(list_commands_result.is_ok());
        let commands = list_commands_result.unwrap();
        assert!(commands.is_empty());

        // delete can be called multiple times
        let delete_command_result = logic.delete_command(command_id);
        assert!(delete_command_result.is_err());
    }

    #[test]
    fn test_handle_update_command_last_used_prop_success() {
        let tmp_dir_result = TempDir::new();
        assert!(tmp_dir_result.is_ok());

        let path = tmp_dir_result
            .unwrap()
            .path()
            .to_string_lossy()
            .into_owned();
        let dal = SqliteDal::new_with_custom_path(path);
        assert!(dal.is_ok());
        let logic = Logic::new(dal.unwrap());

        let command = InternalCommand {
            command: "test_command".to_string(),
            tag: None,
            note: None,
            favourite: false,
        };

        let result = logic.add_command(command.clone());
        assert!(result.is_ok());

        let list_commands_result = logic.list_commands(false, false);
        assert!(list_commands_result.is_ok());
        let commands = list_commands_result.unwrap();
        assert!(commands.len() == 1);
        let command_id = commands.first().unwrap().id;
        let last_used = commands.first().unwrap().last_used;

        // a second gone past so the timestamp will update
        thread::sleep(Duration::from_millis(1000));

        let update_last_used_result = logic.update_command_last_used_prop(command_id);
        assert!(update_last_used_result.is_ok());

        // Verify that the last used property has been updated
        let list_commands_result = logic.list_commands(false, false);
        assert!(list_commands_result.is_ok());
        let commands = list_commands_result.unwrap();
        assert!(commands.len() == 1);

        // last_used has been updated
        assert!(commands.first().unwrap().last_used > last_used);
    }

    #[test]
    fn test_handle_generate_param_success() {
        let tmp_dir_result = TempDir::new();
        assert!(tmp_dir_result.is_ok());

        let path = tmp_dir_result
            .unwrap()
            .path()
            .to_string_lossy()
            .into_owned();
        let dal = SqliteDal::new_with_custom_path(path);
        assert!(dal.is_ok());
        let logic = Logic::new(dal.unwrap());

        let command = InternalCommand {
            command: "echo @{int}".to_string(),
            tag: None,
            note: None,
            favourite: false,
        };

        let result = logic.add_command(command.clone());
        assert!(result.is_ok());

        let list_commands_result = logic.list_commands(false, false);
        assert!(list_commands_result.is_ok());
        let commands = list_commands_result.unwrap();
        assert!(commands.len() == 1);

        let generated_param_result =
            logic.generate_parameters(commands.first().unwrap().internal_command.command.clone());
        assert!(generated_param_result.is_ok());
        let (generated_param, generated_parameters) = generated_param_result.unwrap();
        assert_ne!(generated_param, "echo @{int}");
        assert_eq!(generated_parameters.len(), 1);
    }
}
