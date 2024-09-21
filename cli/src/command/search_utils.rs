use crate::{args::PrintStyle, outputs::ErrorOutput, utils::truncate_string};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use data::models::Command;
use inquire::{InquireError, Select, Text};
use log::error;
use logic::command::{handle_list_commands, handle_search_command, SearchCommandArgs};
use prettytable::{format, Cell, Row, Table};
use termion::terminal_size;
use thiserror::Error;

pub fn display_search_args_wizard(
    alias: &Option<String>,
    command: &Option<String>,
    tag: &Option<String>,
) -> bool {
    alias.is_none() && command.is_none() && tag.is_none()
}

/// Generates a wizard to set the properties for command searching
pub fn search_args_wizard() -> Result<SearchCommandArgs, InquireError> {
    let command = Text::new("Command").prompt()?;

    let alias = Text::new("Alias").prompt()?;

    let tag = Text::new("Tag").prompt()?;

    Ok(SearchCommandArgs {
        alias: if !alias.is_empty() { Some(alias) } else { None },
        command: if !command.is_empty() {
            Some(command)
        } else {
            None
        },
        tag: if !tag.is_empty() { Some(tag) } else { None },
    })
}

#[derive(Error, Debug)]
pub enum GetSelectedItemFromUserError {
    #[error("failed to get commands")]
    GetCommands(#[from] logic::DefaultLogicError),

    #[error("no commands found")]
    NoCommandsFound,

    #[error("failed to get selected item")]
    InquireError(#[from] InquireError),
}

/// Gets search candidates from the database and prompts the user to select one
pub fn get_searched_commands(
    search_args: SearchCommandArgs,
    print_style: PrintStyle,
    display_limit: u32,
) -> Result<Command, GetSelectedItemFromUserError> {
    let commands = match handle_search_command(search_args) {
        Ok(c) => c,
        Err(e) => {
            error!(target: "Search Utils", "Failed to get commands from DB: {:?}", e);
            return Err(GetSelectedItemFromUserError::GetCommands(e));
        }
    };

    let selected_command =
        match get_selected_item_from_user(commands.clone(), print_style, display_limit) {
            Ok(i) => i,
            Err(e) => {
                return Err(e);
            }
        };
    Ok(selected_command)
}

/// Gets all commands from the database in the user's preferred format
/// and prompts the user to select one
pub fn get_listed_commands(
    order_by_use: bool,
    favourite: bool,
    print_style: PrintStyle,
    display_limit: u32,
) -> Result<Command, GetSelectedItemFromUserError> {
    let commands = match handle_list_commands(order_by_use, favourite) {
        Ok(c) => c,
        Err(e) => {
            return Err(GetSelectedItemFromUserError::GetCommands(e));
        }
    };

    let selected_command =
        match get_selected_item_from_user(commands.clone(), print_style, display_limit) {
            Ok(i) => i,
            Err(e) => {
                return Err(e);
            }
        };
    Ok(selected_command)
}

/// Generates a wizard to prompt the user to select a command from a list of commands
fn get_selected_item_from_user(
    commands: Vec<Command>,
    print_style: PrintStyle,
    display_limit: u32,
) -> Result<Command, GetSelectedItemFromUserError> {
    if commands.is_empty() {
        return Err(GetSelectedItemFromUserError::NoCommandsFound);
    }

    let (formatted_commands, columns) = format_commands_for_printing(&commands, print_style);

    println!(); // Spacing
    let selected_command = match Select::new(
        &("Select a command ".to_owned() + columns + ":"),
        formatted_commands,
    )
    // Only display the command once the user makes a selection
    .with_formatter(&|i| commands[i.index].internal_command.command.to_string())
    .with_page_size(display_limit as usize)
    .raw_prompt()
    {
        Ok(c) => c,
        Err(e) => {
            return Err(GetSelectedItemFromUserError::InquireError(e));
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
            "(Alias | Command | Tag | Note | Favourite [YES/NO])",
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
                "YES"
            } else {
                "NO"
            }),
        ]));
    }

    let table_str = table.to_string();
    return table_str.lines().map(|s| s.to_string()).collect();
}

pub fn copy_text(cmd: &str, text_to_copy: String) {
    let mut clipboard = match ClipboardContext::new() {
        Ok(ctx) => ctx,
        Err(e) => {
            error!(target: cmd, "Failed to initialize the clipboard: {:?}", e);
            ErrorOutput::FailedToCopy(text_to_copy).print();
            return;
        }
    };
    match clipboard.set_contents(text_to_copy.clone()) {
        Ok(()) => println!("\nCommand copied to clipboard: {}", text_to_copy),
        Err(e) => {
            error!(target: cmd, "Failed copy command to clipboard: {:?}", e);
            ErrorOutput::FailedToCopy(text_to_copy).print();
        }
    }
}
