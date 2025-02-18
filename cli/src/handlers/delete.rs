use crate::{
    args::SearchArgs,
    handlers::cli_prompter::{
        check_search_args_exist, PromptUserForCommandSelectionError, SearchArgsUserInput,
    },
    Cli,
};
use inquire::InquireError;
use log::error;
use logic::command::{SearchCommandArgs, SearchCommandError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleDeleteError {
    #[error("Failed to get user input")]
    Inquire(#[from] InquireError),
    #[error("No commands found")]
    NoCommandsFound,
    #[error("Failed to search for command")]
    Search(#[from] SearchCommandError),
    #[error("Failed to select a command")]
    SelectCommand(#[from] PromptUserForCommandSelectionError),
    #[error("Failed to delete command")]
    LogicDelete(#[from] logic::command::DeleteCommandError),
}

impl Cli {
    /// CLI handler for the delete command
    pub fn handle_delete_command(&self, args: SearchArgs) -> Result<(), HandleDeleteError> {
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
            return Err(HandleDeleteError::NoCommandsFound);
        }

        let selected_command = self.prompt_user_for_command_selection(search_results)?;
        Ok(self.logic.delete_command(selected_command.id)?)
    }
}
