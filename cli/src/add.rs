use crate::args::AddArgs;
use inquire::{Text, InquireError};

#[derive(Debug)]
pub struct CommandProperties {
    alias: Option<String>,
    tag: Option<String>,
    note: Option<String>,
}

/// Generates a wizard to set the properties of a command
/// 
/// Arguments:
/// - cur_alias: Option<String> - The current alias of the command
/// - cur_note: Option<String> - The current note of the command
/// - cur_tag: Option<String> - The current tag of the command
fn set_command_properties_wizard(command: &str, cur_alias: Option<String>, cur_tag: Option<String>, cur_note: Option<String>) -> Result<CommandProperties, InquireError> {
    let alias = Text::new("Alias (Default is the command text)")
        .with_default(command)
        .with_initial_value(cur_alias.as_deref().unwrap_or(""))
        .prompt()?;

    let tag = Text::new("Tag")
        .with_initial_value(cur_tag.as_deref().unwrap_or(""))
        .prompt()?;

    let note = Text::new("Note")
        .with_initial_value(cur_note.as_deref().unwrap_or(""))
        .prompt()?;

    return Ok(CommandProperties {
        alias: Some(alias),
        tag: if tag != "" {Some(tag)} else { None },
        note: if note != "" {Some(note)} else { None },
    });      
}

pub fn handle_add(args: AddArgs) {
    let command = args.command;
    let mut alias = args.alias;
    let mut tag = args.tag;
    let mut note = args.note;

    if alias.is_none() && tag.is_none() && note.is_none() {
        let command_properties = set_command_properties_wizard(&command, alias, tag, note).unwrap();
        alias = command_properties.alias;
        tag = command_properties.tag;
        note = command_properties.note;
    }

    //TODO - Add the command to the database
}