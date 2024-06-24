use crate::args::AddArgs;
use inquire::{InquireError, Text};
use logic::command::AddCommandParams;

#[derive(Debug)]
/// The properties of a command
struct AddCommandProperties {
    alias: String,
    tag: Option<String>,
    note: Option<String>,
}

/// Generates a wizard to set the properties of a command
fn set_command_properties_wizard(command: &str) -> Result<AddCommandProperties, InquireError> {
    let alias = Text::new("Alias (Default is the command text):")
        .with_default(command)
        .prompt()?;

    let tag = Text::new("Tag:").prompt()?;

    let note = Text::new("Note:").prompt()?;

    return Ok(AddCommandProperties {
        alias: alias,
        tag: if tag != "" { Some(tag) } else { None },
        note: if note != "" { Some(note) } else { None },
    });
}

/// UI handler for the add command
pub fn handle_add_command(args: AddArgs) {
    let command = args.command;
    let mut alias = args.alias;
    let mut tag = args.tag;
    let mut note = args.note;
    let favourite = args.favourite;

    // If no alias, tag, or note is provided, generate a wizard to get them
    if alias.is_none() && tag.is_none() && note.is_none() {
        let command_properties = match set_command_properties_wizard(&command) {
            Ok(properties) => properties,
            Err(e) => {
                println!("Add Cmd: Error setting command properties: {:?}", e);
                return;
            }
        };

        alias = Some(command_properties.alias);
        tag = command_properties.tag;
        note = command_properties.note;
    } else if alias.is_none() {
        // If the alias is not provided, set it equal to the command
        alias = Some(command.clone());
    }

    // Add the command to the database
    let add_result = logic::command::handle_add_command(AddCommandParams {
        command: command,
        alias: alias.unwrap(),
        tag: tag,
        note: note,
        favourite: favourite,
    });

    match add_result {
        Ok(_) => println!("Command added successfully"),
        Err(e) => println!("Add Cmd: Error adding command: {:?}", e),
    }
}
