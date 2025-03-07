use crate::{
    args::AddArgs,
    handlers::CommandInputValidator,
    outputs::{format_output, print_internal_command_table, spacing},
    utils::none_if_empty,
    Cli,
};
use data::models::InternalCommand;
use inquire::error::InquireError;
use inquire::{Select, Text};
use log::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleAddError {
    #[error("Failed to get user input: {0}")]
    Inquire(#[from] InquireError),
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("Failed to initialize logic: {0}")]
    LogicAdd(#[from] logic::command::AddCommandError),
}

impl Cli {
    /// CLI handler for the add command
    pub fn handle_add_command(&self, args: AddArgs) -> Result<(), HandleAddError> {
        let add_args_exist = args.command.is_some();

        let user_input = if !add_args_exist {
            prompt_user_for_add_args(args)?
        } else {
            InternalCommand::try_from(args)?
        };

        self.logic.add_command(user_input.clone())?;

        if add_args_exist {
            // If the user added the command via CLI arguments, we need to
            // display the information so they can confirm the validity
            print_internal_command_table(&user_input);
        }

        Ok(())
    }
}

/// Generates a wizard to set the properties of a command
fn prompt_user_for_add_args(args: AddArgs) -> Result<InternalCommand, InquireError> {
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
