use crate::{
    args::SearchArgs,
    command::search_utils::{
        check_search_args_exist, copy_to_clipboard, CopyTextError,
        PromptUserForCommandSelectionError, SearchArgsUserInput,
    },
    Cli,
};
use inquire::InquireError;
use log::error;
use logic::command::{SearchCommandArgs, SearchCommandError};
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
    LogicParam(#[from] logic::param::ParameterError),
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

        // Generate parameters for the command
        let (text_to_copy, _) = self
            .logic
            .generate_parameters(user_selection.internal_command.command)?;

        copy_to_clipboard(text_to_copy)?;

        Ok(self
            .logic
            .update_command_last_used_prop(user_selection.id)?)
    }
}
