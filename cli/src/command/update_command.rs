use crate::{
    args::SearchAndPrintArgs,
    command::search_utils::{
        check_search_args_exist, fetch_search_candidates, get_search_args_from_user,
        prompt_user_for_command_selection, SearchArgsUserInput,
    },
    outputs::{format_output, Output},
};
use data::models::InternalCommand;
use inquire::{InquireError, Select, Text};
use log::error;
use logic::Logic;
use thiserror::Error;

use super::search_utils::{FetchSearchCandidatesError, PromptUserForCommandSelectionError};

#[derive(Error, Debug)]
pub enum HandleUpdateError {
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
    LogicUpdate(#[from] logic::command::UpdateCommandError),
}

/// Generates a wizard to set the properties of a command
///
/// Arguments:
/// - cur_command: String - The current command text
/// - cur_alias: String - The current alias of the command
/// - cur_note: Option<String> - The current note of the command
/// - cur_tag: Option<String> - The current tag of the command
/// - cur_favourite: bool - The current favourite status of the command
pub fn set_command_properties_wizard(
    cur_command: InternalCommand,
) -> Result<InternalCommand, InquireError> {
    let command = Text::new(&format_output("<bold>Command</bold>:"))
        .with_initial_value(&cur_command.command)
        .prompt()?;

    let alias = Text::new(&format_output("<bold>Alias</bold>:"))
        .with_initial_value(&cur_command.alias)
        .prompt()?;

    let tag = Text::new(&format_output(
        "<bold>Tag</bold> <italics>(Leave blank to skip)</italics><bold>:</bold>",
    ))
    .with_initial_value(&cur_command.tag.unwrap_or(String::from("")))
    .prompt()?;

    let note = Text::new(&format_output(
        "<bold>Note</bold> <italics>(Leave blank to skip)</italics><bold>:</bold>",
    ))
    .with_initial_value(&cur_command.note.unwrap_or(String::from("")))
    .prompt()?;

    let favourite = Select::new(&format_output("<bold>Favourite:</bold>"), vec!["Yes", "No"])
        .with_starting_cursor(if cur_command.favourite { 0 } else { 1 })
        .prompt()?
        == "Yes";

    Ok(InternalCommand {
        command,
        alias,
        tag: if !tag.is_empty() { Some(tag) } else { None },
        note: if !note.is_empty() { Some(note) } else { None },
        favourite,
    })
}

/// UI handler for the update command
pub fn handle_update_command(args: SearchAndPrintArgs) -> Result<(), HandleUpdateError> {
    // Get the arguments used for search
    let search_user_input = if !check_search_args_exist(&args.alias, &args.command, &args.tag) {
        get_search_args_from_user()?
    } else {
        SearchArgsUserInput::from(args.clone())
    };

    // Get the search candidates
    let search_candidates = fetch_search_candidates(search_user_input, args.recent, args.favourite)
        .map_err(|e| match e {
            FetchSearchCandidatesError::NoCommandsFound => HandleUpdateError::NoCommandFound,
            _ => HandleUpdateError::SearchCandidates(e),
        })?;

    // Prompt the user to select a command
    let selected_command =
        prompt_user_for_command_selection(search_candidates, args.print_style, args.display_limit)?;

    // Get the new command properties from the user
    Output::UpdateCommandSectionTitle.print();
    let new_internal_command = set_command_properties_wizard(selected_command.internal_command)
        .map_err(HandleUpdateError::Inquire)?;

    let logic = Logic::try_default()?;

    // Update the selected command
    logic
        .update_command(selected_command.id, new_internal_command)
        .map_err(HandleUpdateError::LogicUpdate)?;

    Output::UpdateCommandSuccess.print();
    Ok(())
}
