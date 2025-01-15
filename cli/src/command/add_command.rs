use crate::{
    args::AddArgs,
    outputs::{format_output, print_internal_command_table, spacing},
};
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
            command: args.command,
            tag: args.tag,
            note: args.note,
            favourite: args.favourite,
        }
    }
}

/// Generates a wizard to set the properties of a command
fn get_add_args_from_user(command: &str) -> Result<InternalCommand, InquireError> {
    spacing();

    let tag = Text::new(&format_output(
        "<bold>Tag</bold> <italics>(Leave blank to skip)</italics><bold>:</bold>",
    ))
    .prompt()?;

    let note = Text::new(&format_output(
        "<bold>Note</bold> <italics>(Leave blank to skip)</italics><bold>:</bold>",
    ))
    .prompt()?;

    let favourite = Select::new(&format_output("<bold>Favourite:</bold>"), vec!["Yes", "No"])
        .with_starting_cursor(1)
        .prompt()?
        == "Yes";

    Ok(InternalCommand {
        command: String::from(command),
        tag: if !tag.is_empty() { Some(tag) } else { None },
        note: if !note.is_empty() { Some(note) } else { None },
        favourite,
    })
}

/// CLI handler for the add command
pub fn handle_add_command(args: AddArgs) -> Result<(), HandleAddError> {
    let add_args_exist = args.tag.is_some() || args.note.is_some();

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
        print_internal_command_table(&internal_command);
    }

    Ok(())
}
