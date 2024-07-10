use crate::{
    args::SearchAndPrintArgs,
    command::search_utils::{
        display_search_args_wizard, get_searched_commands, search_args_wizard,
        GetSelectedItemFromUserError,
    },
};
use data::models::InternalCommand;
use inquire::{InquireError, Select, Text};
use logic::command::SearchCommandArgs;

/// Generates a wizard to set the properties of a command
///
/// Arguments:
/// - cur_command: String - The current command text
/// - cur_alias: String - The current alias of the command
/// - cur_note: Option<String> - The current note of the command
/// - cur_tag: Option<String> - The current tag of the command
/// - cur_favourite: bool - The current favourite status of the command
pub fn set_command_properties_wizard(
    cur_command: String,
    cur_alias: String,
    cur_tag: Option<String>,
    cur_note: Option<String>,
    cur_favourite: bool,
) -> Result<InternalCommand, InquireError> {
    let command = Text::new("Command:")
        .with_initial_value(&cur_command)
        .prompt()?;

    let alias = Text::new("Alias:")
        .with_initial_value(&cur_alias)
        .prompt()?;

    let tag = Text::new("Tag:")
        .with_initial_value(&cur_tag.unwrap_or(String::from("")))
        .prompt()?;

    let note = Text::new("Note:")
        .with_initial_value(&cur_note.unwrap_or(String::from("")))
        .prompt()?;

    let favourite = Select::new("Favourite:", vec!["Yes", "No"])
        .with_starting_cursor(if cur_favourite { 0 } else { 1 })
        .prompt()?
        == "Yes";

    return Ok(InternalCommand {
        command: command,
        alias: alias,
        tag: if tag != "" { Some(tag) } else { None },
        note: if note != "" { Some(note) } else { None },
        favourite: favourite,
    });
}

/// UI handler for the update command
pub fn handle_update_command(args: SearchAndPrintArgs) {
    let mut command = args.command;
    let mut alias = args.alias;
    let mut tag = args.tag;
    let print_style = args.print_style;
    let print_limit = args.display_limit;

    // If no search arguments are provided, generate a wizard to get them
    if display_search_args_wizard(&alias, &command, &tag) {
        let command_properties = match search_args_wizard() {
            Ok(properties) => properties,
            Err(e) => {
                println!("Search Cmd: Error setting command properties: {:?}", e);
                return;
            }
        };

        alias = command_properties.alias;
        tag = command_properties.tag;
        command = command_properties.command;
    }

    // Get the selected command
    let selected_command = match get_searched_commands(
        SearchCommandArgs {
            alias: alias,
            command: command,
            tag: tag,
        },
        print_style,
        print_limit,
    ) {
        Ok(c) => c,
        Err(e) => match e {
            GetSelectedItemFromUserError::NoCommandsFound => {
                println!("No commands found");
                return;
            }
            _ => {
                println!("Update Cmd: Failed to get selected command: {:?}", e);
                return;
            }
        },
    };

    // Get the new command properties from the user
    println!("\nUpdate Command:");
    let new_command_properties = match set_command_properties_wizard(
        selected_command.internal_command.command,
        selected_command.internal_command.alias,
        selected_command.internal_command.tag,
        selected_command.internal_command.note,
        selected_command.internal_command.favourite,
    ) {
        Ok(properties) => properties,
        Err(e) => {
            println!("Update Cmd: Error setting command properties: {:?}", e);
            return;
        }
    };

    // Update the selected command
    match logic::command::handle_update_command(
        selected_command.id,
        InternalCommand {
            alias: new_command_properties.alias,
            command: new_command_properties.command,
            tag: new_command_properties.tag,
            note: new_command_properties.note,
            favourite: new_command_properties.favourite,
        },
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("Update Cmd: Failed to delete command: {:?}", e);
            return;
        }
    };

    println!("\nCommand updated successfully");
}
