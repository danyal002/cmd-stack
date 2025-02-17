use crate::{
    args::SearchArgs,
    handlers::{
        cli_prompter::{
            check_search_args_exist, PromptUserForCommandSelectionError, SearchArgsUserInput,
        },
        CommandInputValidator,
    },
    outputs::{format_output, Output},
    utils::none_if_empty,
    Cli,
};
use data::models::InternalCommand;
use inquire::{InquireError, Select, Text};
use log::error;
use logic::command::{SearchCommandArgs, SearchCommandError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleUpdateError {
    #[error("Failed to get user input")]
    Inquire(#[from] InquireError),
    #[error("No command found")]
    NoCommandFound,
    #[error("Failed to search for command")]
    Search(#[from] SearchCommandError),
    #[error("Failed to select a command")]
    SelectCommand(#[from] PromptUserForCommandSelectionError),
    #[error("Failed to update command")]
    LogicUpdate(#[from] logic::command::UpdateCommandError),
}

/// Generates a wizard to set the properties of a command
///
/// Arguments:
/// - cur_command: String - The current command text
/// - cur_note: Option<String> - The current note of the command
/// - cur_tag: Option<String> - The current tag of the command
/// - cur_favourite: bool - The current favourite status of the command
pub fn prompt_user_for_command(
    cur_command: InternalCommand,
) -> Result<InternalCommand, InquireError> {
    let command = Text::new(&format_output("<bold>Command</bold>:"))
        .with_initial_value(&cur_command.command)
        .with_validator(CommandInputValidator)
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
        tag: none_if_empty(tag),
        note: none_if_empty(note),
        favourite,
    })
}

impl Cli {
    /// UI handler for the update command
    pub fn handle_update_command(&self, args: SearchArgs) -> Result<(), HandleUpdateError> {
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

        // Get the new command properties from the user
        Output::UpdateCommandSectionTitle.print();
        let new_internal_command = prompt_user_for_command(user_selection.internal_command)?;

        Ok(self
            .logic
            .update_command(user_selection.id, new_internal_command)?)
    }
}
