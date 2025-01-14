use crate::{
    args::{PrintStyle, SearchAndPrintArgs},
    outputs::{format_output, spacing, Output},
    utils::truncate_string,
};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use data::models::Command;
use inquire::{InquireError, Select, Text};
use log::error;
use logic::{command::SearchCommandArgs, Logic};
use prettytable::{format, Cell, Row, Table};
use termion::terminal_size;
use thiserror::Error;

pub struct SearchArgsUserInput {
    pub alias: Option<String>,
    pub command: Option<String>,
    pub tag: Option<String>,
}
impl From<SearchAndPrintArgs> for SearchArgsUserInput {
    fn from(args: SearchAndPrintArgs) -> Self {
        SearchArgsUserInput {
            alias: args.alias,
            command: args.command,
            tag: args.tag,
        }
    }
}

/// Given the user input for `alias`, `command` and `tag`, determine
/// if the user provided search arguments
///
/// Returns a boolean
pub fn check_search_args_exist(
    alias: &Option<String>,
    command: &Option<String>,
    tag: &Option<String>,
) -> bool {
    alias.is_some() || command.is_some() || tag.is_some()
}

/// Generates a wizard to set the properties for command searching
pub fn get_search_args_from_user() -> Result<SearchArgsUserInput, InquireError> {
    spacing();

    let command = Text::new(&format_output(
        "<bold>Command</bold> <italics>(Leave blank for no filter)</italics><bold>:</bold>",
    ))
    .prompt()?;

    let alias = Text::new(&format_output(
        "<bold>Alias</bold> <italics>(Leave blank for no filter)</italics><bold>:</bold>",
    ))
    .prompt()?;

    let tag = Text::new(&format_output(
        "<bold>Tag</bold> <italics>(Leave blank for no filter)</italics><bold>:</bold>",
    ))
    .prompt()?;

    Ok(SearchArgsUserInput {
        alias: Some(alias),
        command: Some(command),
        tag: Some(tag),
    })
}

#[derive(Error, Debug)]
pub enum FetchSearchCandidatesError {
    #[error("No commands found")]
    NoCommandsFound,
    #[error("failed to initialize logic")]
    LogicInit(#[source] logic::LogicInitError),
    #[error("failed to search for commands")]
    SearchCommands(#[source] logic::command::SearchCommandError),
}

/// Gets search candidates from the database
pub fn fetch_search_candidates(
    search_args: SearchArgsUserInput,
    order_by_use: bool,
    favourites_only: bool,
) -> Result<Vec<Command>, FetchSearchCandidatesError> {
    let logic = match Logic::try_default() {
        Ok(l) => l,
        Err(e) => {
            error!(target: "Search Utils", "Failed to initialize logic: {:?}", e);
            return Err(FetchSearchCandidatesError::LogicInit(e));
        }
    };

    let commands = match logic.search_command(SearchCommandArgs {
        alias: search_args.alias,
        command: search_args.command,
        tag: search_args.tag,
        order_by_use,
        favourites_only,
    }) {
        Ok(c) => c,
        Err(e) => {
            error!(target: "Search Utils", "Failed to search for commands: {:?}", e);
            return Err(FetchSearchCandidatesError::SearchCommands(e));
        }
    };

    if commands.len() == 0 {
        return Err(FetchSearchCandidatesError::NoCommandsFound);
    }

    Ok(commands)
}

#[derive(Error, Debug)]
pub enum PromptUserForCommandSelectionError {
    #[error("No commands found")]
    NoCommandsFound,
    #[error("failed to get selected item")]
    InquireError(#[from] InquireError),
}

/// Handles the UI interaction to prompt the user for selection
///
/// `commands` must be non-empty
pub fn prompt_user_for_command_selection(
    commands: Vec<Command>,
    print_style: PrintStyle,
    display_limit: u32,
) -> Result<Command, PromptUserForCommandSelectionError> {
    if commands.is_empty() {
        return Err(PromptUserForCommandSelectionError::NoCommandsFound);
    }

    let (formatted_commands, columns) = format_commands_for_printing(&commands, print_style);

    spacing();
    let selected_command = match Select::new(
        &format_output(
            &("<bold>Select a command</bold> <italics>".to_owned()
                + columns
                + "</italics><bold>:</bold>"),
        ),
        formatted_commands,
    )
    // Only display the command once the user makes a selection
    .with_formatter(&|i| commands[i.index].internal_command.command.to_string())
    .with_page_size(display_limit as usize)
    .raw_prompt()
    {
        Ok(c) => c,
        Err(e) => {
            return Err(PromptUserForCommandSelectionError::InquireError(e));
        }
    };

    Ok(commands[selected_command.index].clone())
}

/// Formats the commands for printing based on the user's preferred style.
/// Returns the columns to be printed
fn format_commands_for_printing(
    commands: &Vec<Command>,
    print_style: PrintStyle,
) -> (Vec<String>, &str) {
    match print_style {
        PrintStyle::All => (
            format_internal_commands(commands),
            "(Alias | Command | Tag | Note | Favourite [*])",
        ),
        PrintStyle::Alias => (
            commands
                .iter()
                .map(|c| c.internal_command.alias.clone())
                .collect(),
            "(Alias)",
        ),
        PrintStyle::Command => (
            commands
                .iter()
                .map(|c| c.internal_command.command.clone())
                .collect(),
            "(Command)",
        ),
    }
}

fn format_internal_commands(commands: &Vec<Command>) -> Vec<String> {
    let (width, _) = terminal_size().unwrap_or((150, 0)); // Default to 150 if terminal size cannot be determined

    // Define maximum widths for each column
    let alias_width = std::cmp::max(width * 15 / 100, 12) as i32; // Alias gets 15% of width or 12, whichever is more
    let tag_width = std::cmp::max(width * 5 / 100, 8) as i32; // Tag gets 5% of the width or 8, whichever is more
    let favourite_width = 5;

    let remaining_width = std::cmp::max(
        width as i32 - alias_width - tag_width - favourite_width - 12,
        0,
    );
    let command_width = remaining_width * 75 / 100; // Commands get 75% of remaining width
    let note_width = remaining_width - command_width;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    for command in commands {
        let truncated_alias =
            truncate_string(&command.internal_command.alias, alias_width as usize);
        let truncated_tag = truncate_string(
            command.internal_command.tag.as_deref().unwrap_or(""),
            tag_width as usize,
        );
        let truncated_command =
            truncate_string(&command.internal_command.command, command_width as usize);
        let truncated_note = truncate_string(
            command.internal_command.note.as_deref().unwrap_or(""),
            note_width as usize,
        );

        table.add_row(Row::new(vec![
            Cell::new(&truncated_alias),
            Cell::new(&truncated_command),
            Cell::new(&truncated_tag),
            Cell::new(&truncated_note),
            Cell::new(if command.internal_command.favourite {
                "*"
            } else {
                ""
            }),
        ]));
    }

    let table_str = table.to_string();
    table_str.lines().map(|s| s.to_string()).collect()
}

#[derive(Error, Debug)]
pub enum CopyTextError {
    #[error("Failed to initialize the clipboard")]
    ClipboardInit,

    #[error("Failed to copy text to clipboard")]
    Copy,
}

pub fn copy_text(cmd: &str, text_to_copy: String) -> Result<(), CopyTextError> {
    let mut clipboard = ClipboardContext::new().map_err(|e| {
        error!(target: cmd, "Failed to initialize the clipboard: {:?}", e);
        CopyTextError::ClipboardInit
    })?;

    clipboard.set_contents(text_to_copy.clone()).map_err(|e| {
        error!(target: cmd, "Failed copy command to clipboard: {:?}", e);
        CopyTextError::Copy
    })?;

    Output::CommandCopiedToClipboard(text_to_copy).print();
    Ok(())
}
