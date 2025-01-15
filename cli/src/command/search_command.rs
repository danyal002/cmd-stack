use crate::{
    args::SearchAndPrintArgs,
    command::search_utils::{
        check_search_args_exist, copy_to_clipboard, fetch_search_candidates,
        get_search_args_from_user, prompt_user_for_command_selection, CopyTextError,
        FetchSearchCandidatesError, PromptUserForCommandSelectionError, SearchArgsUserInput,
    },
};
use inquire::InquireError;
use log::error;
use logic::Logic;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleSearchError {
    #[error("Failed to get user input")]
    Inquire(#[from] InquireError),
    #[error("No command found")]
    NoCommandFound,
    #[error("Failed to get search candidates")]
    SearchCandidates(#[from] FetchSearchCandidatesError),
    #[error("Failed to select a command")]
    SelectCommand(#[from] PromptUserForCommandSelectionError),
    #[error("Failed to copy command")]
    Copy(#[from] CopyTextError),
    #[error("Failed to initialize logic")]
    LogicInit(#[from] logic::LogicInitError),
    #[error("Failed to generate parameters")]
    LogicParam(#[from] logic::param::ParameterError),
    #[error("Failed to update command")]
    LogicUpdate(#[from] logic::command::UpdateCommandError),
}

/// UI handler for the search command
pub fn handle_search_commands(args: SearchAndPrintArgs) -> Result<(), HandleSearchError> {
    // Get the arguments used for search
    let search_user_input = if !check_search_args_exist(&args.command, &args.tag) {
        get_search_args_from_user()?
    } else {
        SearchArgsUserInput::from(args.clone())
    };

    // Get the search candidates
    let search_candidates = fetch_search_candidates(search_user_input, args.recent, args.favourite)
        .map_err(|e| match e {
            FetchSearchCandidatesError::NoCommandsFound => HandleSearchError::NoCommandFound,
            _ => HandleSearchError::SearchCandidates(e),
        })?;

    // Prompt the user to select a command
    let selected_command =
        prompt_user_for_command_selection(search_candidates, args.print_style, args.display_limit)?;

    let logic = Logic::try_default()?;

    // Generate parameters for the command
    let text_to_copy = logic.generate_parameters(selected_command.clone())?;

    // Copy the selected command to the clipboard
    copy_to_clipboard(text_to_copy)?;

    // Update the last used timestamp for the command
    logic.update_command_last_used_prop(selected_command.id)?;

    Ok(())
}
