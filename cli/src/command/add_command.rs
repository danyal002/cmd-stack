use crate::{
    args::AddArgs,
    outputs::{format_output, print_internal_command_table, spacing},
    utils::none_if_empty,
    
};
use data::models::InternalCommand;
use inquire::error::InquireError;
use inquire::{Select, Text};
use log::error;
use logic::Logic;
use thiserror::Error;

use super::CommandInputValidator;

#[derive(Error, Debug)]
pub enum HandleAddError {
    #[error("Failed to get user input")]
    Inquire(#[from] InquireError),
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("Failed to initialize logic")]
    LogicInit(#[from] logic::LogicInitError),
    #[error("Failed to add command")]
    LogicAdd(#[from] logic::command::AddCommandError),
}

impl TryFrom<AddArgs> for InternalCommand {
    type Error = HandleAddError;

    fn try_from(args: AddArgs) -> Result<Self, Self::Error> {
        if let Some(command) = args.command {
            Ok(InternalCommand {
                command,
                tag: args.tag,
                note: args.note,
                favourite: args.favourite,
            })
        } else {
            Err(HandleAddError::MissingField("command".to_string()))
        }
    }
}

/// Generates a wizard to set the properties of a command
fn get_add_args_from_user(args: AddArgs) -> Result<InternalCommand, InquireError> {
    spacing();
    // No check needed since wizard is only displayed if the command field is not present
    let command = Text::new(&format_output("<bold>Command:</bold>"))
        .with_validator(CommandInputValidator)
        .prompt()?;

    let tag = Text::new(&format_output(
        "<bold>Tag</bold> <italics>(Leave blank to skip)</italics><bold>:</bold>",
    ))
    .with_initial_value(&args.tag.unwrap_or_default())
    .prompt()?;

    let note = Text::new(&format_output(
        "<bold>Note</bold> <italics>(Leave blank to skip)</italics><bold>:</bold>",
    ))
    .with_initial_value(&args.note.unwrap_or_default())
    .prompt()?;

    let favourite = Select::new(&format_output("<bold>Favourite:</bold>"), vec!["Yes", "No"])
        .with_starting_cursor(!args.favourite as usize)
        .prompt()?
        == "Yes";

    Ok(InternalCommand {
        command,
        tag: none_if_empty(tag),
        note: none_if_empty(note),
        favourite,
    })
}

/// CLI handler for the add command
pub fn handle_add_command(args: AddArgs) -> Result<(), HandleAddError> {
    let add_args_exist = args.command.is_some();

    // Get the command to add either from CLI args or user input
    let internal_command = if !add_args_exist {
        get_add_args_from_user(args)?
    } else {
        InternalCommand::try_from(args)?
    };

    let logic = Logic::try_default()?;

    // Write the command to the db
    logic.add_command(internal_command.clone())?;

    if add_args_exist {
        // If the user added the command via CLI arguments, we need to
        // display the information so they can confirm the validity
        print_internal_command_table(&internal_command);
    }

    Ok(())
}
