use crate::{args::PrintStyle, outputs::ErrorOutput};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use data::models::Command;
use inquire::{InquireError, Select, Text};
use log::error;
use logic::command::{handle_list_commands, handle_search_command, SearchCommandArgs};
use prettytable::{format, Cell, Row, Table};
use thiserror::Error;

pub fn display_search_args_wizard(
    alias: &Option<String>,
    command: &Option<String>,
    tag: &Option<String>,
) -> bool {
    return alias.is_none() && command.is_none() && tag.is_none();
}

/// Generates a wizard to set the properties for command searching
pub fn search_args_wizard() -> Result<SearchCommandArgs, InquireError> {
    let command = Text::new("Command").prompt()?;

    let alias = Text::new("Alias").prompt()?;

    let tag = Text::new("Tag").prompt()?;

    return Ok(SearchCommandArgs {
        alias: if alias != "" { Some(alias) } else { None },
        command: if command != "" { Some(command) } else { None },
        tag: if tag != "" { Some(tag) } else { None },
    });
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
    return Ok(selected_command);
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
    return Ok(selected_command);
}

/// Generates a wizard to prompt the user to select a command from a list of commands
fn get_selected_item_from_user(
    commands: Vec<Command>,
    print_style: PrintStyle,
    display_limit: u32,
) -> Result<Command, GetSelectedItemFromUserError> {
    if commands.len() == 0 {
        return Err(GetSelectedItemFromUserError::NoCommandsFound);
    }

    let (formatted_commands, columns) = format_commands_for_printing(&commands, print_style);

    println!(); // Spacing
    let selected_command = match Select::new(
        &("Select a command ".to_owned() + columns + ":"),
        formatted_commands,
    )
    // Only display the command once the user makes a selection
    .with_formatter(&|i| format!("{}", &commands[i.index].internal_command.command))
    .with_page_size(display_limit as usize)
    .raw_prompt()
    {
        Ok(c) => c,
        Err(e) => {
            return Err(GetSelectedItemFromUserError::InquireError(e));
        }
    };

    return Ok(commands[selected_command.index].clone());
}

/// Formats the commands for printing based on the user's preferred style.
/// Returns the columns to be printed
fn format_commands_for_printing(
    commands: &Vec<Command>,
    print_style: PrintStyle,
) -> (Vec<String>, &str) {
    return match print_style {
        PrintStyle::All => (
            format_internal_commands(commands),
            "(Alias | Command | Tag | Note | Favourite [YES/NO])",
        ),
        PrintStyle::Alias => (
            commands
                .into_iter()
                .map(|c| c.internal_command.alias.clone())
                .collect(),
            "(Alias)",
        ),
        PrintStyle::Command => (
            commands
                .into_iter()
                .map(|c| c.internal_command.command.clone())
                .collect(),
            "(Command)",
        ),
    };
}

fn format_internal_commands(commands: &Vec<Command>) -> Vec<String> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    for command in commands {
        table.add_row(Row::new(vec![
            Cell::new(&command.internal_command.alias),
            Cell::new(&command.internal_command.command),
            Cell::new(command.internal_command.tag.as_deref().unwrap_or("")),
            Cell::new(command.internal_command.note.as_deref().unwrap_or("")),
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
            return;
        }
    }
}
