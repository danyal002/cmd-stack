use crate::{
    args::SearchAndPrintArgs,
    command::search_utils::{
        check_search_args_exist, fetch_search_candidates, get_search_args_from_user,
        prompt_user_for_command_selection, FetchSearchCandidatesError,
        PromptUserForCommandSelectionError, SearchArgsUserInput,
    },
    outputs::Output,
};
use inquire::InquireError;
use log::error;
use logic::Logic;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleDeleteError {
    #[error("Failed to get user input")]
    Inquire(#[from] InquireError),
    #[error("No command found")]
    NoCommandFound,
    #[error("Failed to get search candidates")]
    SearchCandidates(#[from] FetchSearchCandidatesError),
    #[error("Failed to select a command")]
    SelectCommand(#[from] PromptUserForCommandSelectionError),
    #[error("Failed to initialize logic")]
    LogicInit(#[from] logic::LogicInitError),
    #[error("Failed to delete command")]
    LogicDelete(#[from] logic::command::DeleteCommandError),
}

/// CLI handler for the delete command
pub fn handle_delete_command(args: SearchAndPrintArgs) -> Result<(), HandleDeleteError> {
    // Get the arguments used for search
    let search_user_input = if !check_search_args_exist(&args.alias, &args.command, &args.tag) {
        get_search_args_from_user()?
    } else {
        SearchArgsUserInput::from(args.clone())
    };

    // Get the search candidates
    let search_candidates = fetch_search_candidates(search_user_input, args.recent, args.favourite)
        .map_err(|e| match e {
            FetchSearchCandidatesError::NoCommandsFound => HandleDeleteError::NoCommandFound,
            _ => HandleDeleteError::SearchCandidates(e),
        })?;

    // Prompt the user to select a command
    let selected_command = prompt_user_for_command_selection(
        search_candidates,
        args.print_style,
        args.display_limit.clone(),
    )?;

    let logic = Logic::try_default()?;

    // Delete the selected command
    logic.delete_command(selected_command.id)?;

    Output::DeleteCommandSuccess.print();
    Ok(())
}
