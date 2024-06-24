use crate::args::AddArgs;
use inquire::{InquireError, Text};
use logic::command::{handle_add_command, AddCommandParams};

#[derive(Debug)]
pub struct CommandProperties {
    alias: String,
    tag: Option<String>,
    note: Option<String>,
}

#[derive(Debug)]
pub struct AddCommandRequest {

}

/// Generates a wizard to set the properties of a command
///
/// Arguments:
/// - cur_alias: Option<String> - The current alias of the command
/// - cur_note: Option<String> - The current note of the command
/// - cur_tag: Option<String> - The current tag of the command
fn set_command_properties_wizard(command: &str) -> Result<CommandProperties, InquireError> {
    let alias = Text::new("Alias (Default is the command text)")
        .with_default(command)
        .prompt()?;

    let tag = Text::new("Tag").prompt()?;

    let note = Text::new("Note").prompt()?;

    return Ok(CommandProperties {
        alias: alias,
        tag: if tag != "" { Some(tag) } else { None },
        note: if note != "" { Some(note) } else { None },
    });
}

pub async fn handle_add(args: AddArgs) {
    let command = args.command;
    let mut alias = args.alias;
    let mut tag = args.tag;
    let mut note = args.note;

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

    let add_result = handle_add_command(AddCommandParams {
        command: command,
        alias: alias.unwrap(),
        tag: tag,
        note: note,
    }).await;

    match add_result {
        Ok(_) => println!("Command added successfully"),
        Err(e) => println!("Add Cmd: Error adding command: {:?}", e),
    }
}
