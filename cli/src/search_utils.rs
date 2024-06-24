use crate::args::PrintStyle;
use data::models::Command;
use inquire::{InquireError, Select, Text};
use logic::command::{handle_list_commands, handle_search_command, SearchCommandArgs};
use thiserror::Error;

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
    GetCommands(#[from] logic::command::SearchCommandError),

    #[error("no commands found")]
    NoCommandsFound,

    #[error("failed to get selected item")]
    InquireError(#[from] InquireError),
}

/// Gets search candidates for the user from the database and prompts the user to select one
pub fn get_searched_commands(
    search_args: SearchCommandArgs,
    print_style: PrintStyle,
    display_limit: u32,
) -> Result<Command, GetSelectedItemFromUserError> {
    let commands = match handle_search_command(search_args) {
        Ok(c) => c,
        Err(e) => {
            println!("Search: Failed to get commands from DB: {:?}", e);
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

    let selected_command = match Select::new(
        "Select a command:",
        format_commands_for_printing(&commands, print_style),
    )
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

/// Formats the commands for printing based on the user's preferred style
fn format_commands_for_printing(commands: &Vec<Command>, print_style: PrintStyle) -> Vec<String> {
    return match print_style {
        PrintStyle::All => commands
            .into_iter()
            .map(|c| c.internal_command.command.clone() + " | " + &c.internal_command.alias)
            .collect(),
        PrintStyle::Alias => commands
            .into_iter()
            .map(|c| c.internal_command.alias.clone())
            .collect(),
        PrintStyle::Command => commands
            .into_iter()
            .map(|c| c.internal_command.command.clone())
            .collect(),
    };
}
