use crate::{
    args::SearchArgs,
    handlers::cli_prompter::{
        check_search_args_exist, copy_to_clipboard, CopyTextError,
        PromptUserForCommandSelectionError, SearchArgsUserInput,
    },
    outputs::spacing,
    Cli,
};
use inquire::{InquireError, Text};
use log::error;
use logic::{
    command::{SearchCommandArgs, SearchCommandError},
    parameters::parser::SerializableParameter,
};
use std::{os::unix::process::CommandExt, process::Command};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleSearchError {
    #[error("Failed to get user input")]
    Inquire(#[from] InquireError),
    #[error("No command found")]
    NoCommandFound,
    #[error("Failed to search for command")]
    Search(#[from] SearchCommandError),
    #[error("Failed to select a command")]
    SelectCommand(#[from] PromptUserForCommandSelectionError),
    #[error("Failed to copy command")]
    Copy(#[from] CopyTextError),
    #[error("Failed to generate parameters")]
    LogicParam(#[from] logic::parameters::ParameterError),
    #[error("Failed to update command")]
    LogicUpdate(#[from] logic::command::UpdateCommandError),
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

        let user_selection = self.prompt_user_for_command_selection(search_results)?;

        let (non_param_strings, parsed_params) = self
            .logic
            .parse_parameters(user_selection.internal_command.command)?;

        // TODO: Display this to the user in a better way. If you have 3 blank parameters, you might lose track of which one you're on.
        // One way to solve this problem would be to display the command to the user with those blanks filled in as we go.
        // Another way would be that we start putting in numbers in the blank parameter like `aws s3 ls s3://@{0}/bucket/my-{1}-bucket`
        // I think a combination of both would be a good solution
        let blank_values: Vec<String> = parsed_params
            .iter()
            .enumerate()
            .filter_map(|(i, param)| match param {
                SerializableParameter::Blank => Some(
                    Text::new(&format!("Enter value for blank parameter #{}:", i + 1)).prompt(),
                ),
                _ => None,
            })
            .collect::<Result<Vec<_>, _>>()?;

        let (text_to_copy, _) =
            self.logic
                .populate_parameters(non_param_strings, parsed_params, blank_values, None)?;

        spacing();

        // Prompt the user to edit the generated command
        let user_edited_cmd = self.prompt_user_for_command_edit(&text_to_copy)?;

        // Prompt the user for command action
        let action = self.prompt_user_for_action()?;

        if action == "Execute" {
            // Note: using `.exec()` will shutdown our app and execute the command, therefore, we can't handle errors.
            let _ = self.logic.update_command_last_used_prop(user_selection.id);
            let _ = Command::new("sh")
                .arg("-c")
                .arg(user_edited_cmd.clone())
                .exec();
        }

        copy_to_clipboard(user_edited_cmd)?;

        Ok(self
            .logic
            .update_command_last_used_prop(user_selection.id)?)
    }
}
