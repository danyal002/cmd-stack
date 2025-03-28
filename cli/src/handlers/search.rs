use crate::{
    args::SearchArgs,
    handlers::cli_prompter::{
        check_search_args_exist, copy_to_clipboard, CopyTextError,
        PromptUserForCommandSelectionError, SearchArgsUserInput,
    },
    outputs::spacing,
    Cli,
};
use inquire::InquireError;
use log::error;
use logic::{
    command::{SearchCommandArgs, SearchCommandError},
    parameters::parser::SerializableParameter,
};
use std::{os::unix::process::CommandExt, process::Command};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleSearchError {
    #[error("Failed to get user input: {0}")]
    Inquire(#[from] InquireError),
    #[error("No command found")]
    NoCommandFound,
    #[error("Failed to search for command: {0}")]
    Search(#[from] SearchCommandError),
    #[error("Failed to select a command: {0}")]
    SelectCommand(#[from] PromptUserForCommandSelectionError),
    #[error("Failed to copy command: {0}")]
    Copy(#[from] CopyTextError),
    #[error("Failed to generate parameters: {0}")]
    LogicParam(#[from] logic::parameters::ParameterError),
    #[error("Failed to update command: {0}")]
    LogicUpdate(#[from] logic::command::UpdateCommandError),
    #[error("Failed to execute command in terminal: {0}")]
    ExecuteCommandInTerminal(String),
}

impl Cli {
    /// UI handler for the search command
    pub fn handle_search_command(&self, args: SearchArgs) -> Result<(), HandleSearchError> {
        // Get the arguments used for search
        let search_user_input = if !check_search_args_exist(&args.command, &args.tag) {
            self.prompt_user_for_search_args()?
        } else {
            SearchArgsUserInput::from(args.clone())
        };
        let search_results = self.logic.search_command(SearchCommandArgs {
            command: search_user_input.command,
            tag: search_user_input.tag,
            order_by_recently_used: args.order_by_recently_used,
            favourites_only: args.favourite,
        })?;
        if search_results.is_empty() {
            return Err(HandleSearchError::NoCommandFound);
        }

        let user_selection = self.prompt_user_for_command_selection(search_results)?;

        let (non_param_strings, parsed_params) = self
            .logic
            .parse_parameters(user_selection.internal_command.command.clone())?;

        let has_blank_params = parsed_params
            .iter()
            .any(|item| matches!(item, SerializableParameter::Blank));
        let blank_param_values = if has_blank_params {
            self.fill_blank_params(&parsed_params)?
        } else {
            spacing();
            Vec::new()
        };

        let (text_to_copy, _) = self.logic.populate_parameters(
            non_param_strings,
            parsed_params,
            blank_param_values,
            None,
        )?;

        // Prompt the user to edit the generated command
        let user_edited_cmd = self.prompt_user_for_command_edit(&text_to_copy)?;

        // Prompt the user for command action
        let action = self.prompt_user_for_action()?;

        if action == "Execute" {
            let _ = self.logic.update_command_last_used_prop(user_selection.id);
            // Note: using `.exec()` will shutdown our app and execute the command if successful.
            return Err(HandleSearchError::ExecuteCommandInTerminal(
                Command::new("sh")
                    .args(["-c", &user_edited_cmd])
                    .exec()
                    .to_string(),
            ));
        } else {
            copy_to_clipboard(user_edited_cmd)?;
        }

        Ok(self
            .logic
            .update_command_last_used_prop(user_selection.id)?)
    }
}
