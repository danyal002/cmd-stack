use crate::{args::AddArgs, command::print_internal_command};
use data::models::InternalCommand;
use inquire::{InquireError, Select, Text};
use log::error;
use logic::Logic;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleAddError {
    #[error("Failed to get user input")]
    Inquire(#[from] InquireError),
    #[error("Failed to initialize logic")]
    LogicInit(#[from] logic::LogicInitError),
    #[error("Failed to add command")]
    LogicAdd(#[from] logic::command::AddCommandError),
}

impl From<AddArgs> for InternalCommand {
    fn from(args: AddArgs) -> Self {
        InternalCommand {
            alias: args.alias.unwrap_or(String::from("")),
            command: args.command,
            tag: args.tag,
            note: args.note,
            favourite: args.favourite,
        }
    }
}

/// Generates a wizard to set the properties of a command
fn get_add_args_from_user(command: &str) -> Result<InternalCommand, InquireError> {
    let alias = Text::new("Alias (Default is the command text):")
        .with_default(command)
        .prompt()?;

    let tag = Text::new("Tag:").prompt()?;

    let note = Text::new("Note:").prompt()?;

    let favourite = Select::new("Favourite:", vec!["Yes", "No"])
        .with_starting_cursor(1)
        .prompt()?
        == "Yes";

    Ok(InternalCommand {
        alias,
        command: String::from(command),
        tag: if !tag.is_empty() { Some(tag) } else { None },
        note: if !note.is_empty() { Some(note) } else { None },
        favourite,
    })
}

/// CLI handler for the add command
pub fn handle_add_command(args: AddArgs) -> Result<(), HandleAddError> {
    let add_args_exist = args.alias.is_some() || args.tag.is_some() || args.note.is_some();

    // Get the command to add either from CLI args or user input
    let internal_command = if !add_args_exist {
        get_add_args_from_user(&args.command)?
    } else {
        InternalCommand::from(args)
    };

    let logic = Logic::try_default()?;

    // Write the command to the db
    logic.add_command(internal_command.clone())?;

    if add_args_exist {
        // If the user added the command via CLI arguments, we need to
        // display the information so they can confirm the validity
        print_internal_command(&internal_command);
    }
    println!("\nCommand added!");
    Ok(())
}
