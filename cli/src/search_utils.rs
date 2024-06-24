use data::models::InternalCommand;
use inquire::{InquireError, Text, Select};
use logic::command::{handle_search_command, SearchCommandArgs};
use thiserror::Error;
use crate::args::PrintStyle;

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

pub struct GetSelectedItemFromUserArgs {
    pub search_args: SearchCommandArgs,
    pub print_style: PrintStyle,
    pub display_limit: u32,
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

pub fn get_selected_item_from_user(args: GetSelectedItemFromUserArgs) -> Result<InternalCommand, GetSelectedItemFromUserError> {
    let commands = match handle_search_command(args.search_args) {
        Ok(c) => c,
        Err(e) => {
        println!("Search: Failed to get commands from DB: {:?}", e);
            return Err(GetSelectedItemFromUserError::GetCommands(e));
        }
    };

    if commands.len() == 0 {
        return Err(GetSelectedItemFromUserError::NoCommandsFound);
    }

    let selected_command = match Select::new("Select a command", format_commands_for_printing(&commands, args.print_style))
        .with_page_size(args.display_limit as usize)
        .raw_prompt() {
        Ok(c) => c,
        Err(e) => {
            return Err(GetSelectedItemFromUserError::InquireError(e));
        }
    };

    return Ok(commands[selected_command.index].clone());
}

fn format_commands_for_printing(commands: &Vec<InternalCommand>, print_style: PrintStyle) -> Vec<String> {
    return match print_style {
        PrintStyle::All => commands.into_iter().map(|c| c.command.clone() + " | " + &c.alias).collect(),
        PrintStyle::Alias => commands.into_iter().map(|c| c.alias.clone()).collect(),
        PrintStyle::Command => commands.into_iter().map(|c| c.command.clone()).collect(),
    };
}