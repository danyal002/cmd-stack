use crate::{args::AddArgs, command::print_internal_command, outputs::ErrorOutput};
use data::models::InternalCommand;
use inquire::{InquireError, Select, Text};
use log::error;

#[derive(Debug)]
struct AddCommandProperties {
    alias: String,
    tag: Option<String>,
    note: Option<String>,
    favourite: bool,
}

/// Generates a wizard to set the properties of a command
fn set_command_properties_wizard(command: &str) -> Result<AddCommandProperties, InquireError> {
    let alias = Text::new("Alias (Default is the command text):")
        .with_default(command)
        .prompt()?;

    let tag = Text::new("Tag:").prompt()?;

    let note = Text::new("Note:").prompt()?;

    let favourite = Select::new("Favourite:", vec!["Yes", "No"])
        .with_starting_cursor(1)
        .prompt()?
        == "Yes";

    return Ok(AddCommandProperties {
        alias: alias,
        tag: if tag != "" { Some(tag) } else { None },
        note: if note != "" { Some(note) } else { None },
        favourite: favourite,
    });
}

/// UI handler for the add command
pub fn handle_add_command(args: AddArgs) {
    let command = args.command;
    let mut alias = args.alias;
    let mut tag = args.tag;
    let mut note = args.note;
    let mut favourite = args.favourite;

    // If no alias, tag, or note is provided, generate a wizard to get them
    let generate_command_with_wizard = alias.is_none() && tag.is_none() && note.is_none();
    if generate_command_with_wizard {
        let command_properties = match set_command_properties_wizard(&command) {
            Ok(properties) => properties,
            Err(e) => {
                error!(target: "Add Cmd", "Error setting command properties: {:?}", e);
                ErrorOutput::UserInput.print();
                return;
            }
        };

        alias = Some(command_properties.alias);
        tag = command_properties.tag;
        note = command_properties.note;
        favourite = command_properties.favourite;
    } else if alias.is_none() {
        // If the alias is not provided, set it equal to the command
        alias = Some(command.clone());
    }

    let alias = match alias {
        Some(a) => a,
        None => {
            error!(target: "Add Cmd", "Could not set alias");
            ErrorOutput::UserInput.print();
            return;
        }
    };

    let internal_command = InternalCommand {
        command: command,
        alias: alias,
        tag: tag,
        note: note,
        favourite: favourite,
    };

    match logic::command::handle_add_command(internal_command.clone()) {
        Ok(_) => {
            if !generate_command_with_wizard {
                // If the user added the command via CLI arguments, we need to
                // display the information so they can confirm the validity
                println!("\nCommand added successfully:");
                print_internal_command(&internal_command);
            } else {
                println!("\nCommand added successfully");
            }
        }
        Err(e) => {
            error!(target: "Add Cmd", "Error adding command: {:?}", e);
            ErrorOutput::AddCmd.print();
        }
    }
}
