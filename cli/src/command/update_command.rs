use crate::{
    args::SearchAndPrintArgs,
    command::search_utils::{
        display_search_args_wizard, get_searched_commands, search_args_wizard,
        GetSelectedItemFromUserError,
    },
    outputs::{process_text_for_output, ErrorOutput},
};
use data::models::InternalCommand;
use inquire::{InquireError, Select, Text};
use log::error;
use logic::{command::SearchCommandArgs, Logic};

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
    let command = Text::new(&process_text_for_output("<bold>Command</bold>:"))
        .with_initial_value(&cur_command)
        .prompt()?;

    let alias = Text::new(&process_text_for_output("<bold>Alias</bold>:"))
        .with_initial_value(&cur_alias)
        .prompt()?;

    let tag = Text::new(&process_text_for_output(
        "<bold>Tag</bold> <italics>(Leave blank to skip)</italics><bold>:</bold>",
    ))
    .with_initial_value(&cur_tag.unwrap_or(String::from("")))
    .prompt()?;

    let note = Text::new(&process_text_for_output(
        "<bold>Note</bold> <italics>(Leave blank to skip)</italics><bold>:</bold>",
    ))
    .with_initial_value(&cur_note.unwrap_or(String::from("")))
    .prompt()?;

    let favourite = Select::new(
        &process_text_for_output("<bold>Favourite:</bold>"),
        vec!["Yes", "No"],
    )
    .with_starting_cursor(if cur_favourite { 0 } else { 1 })
    .prompt()?
        == "Yes";

    Ok(InternalCommand {
        command,
        alias,
        tag: if !tag.is_empty() { Some(tag) } else { None },
        note: if !note.is_empty() { Some(note) } else { None },
        favourite,
    })
}

/// UI handler for the update command
pub fn handle_update_command(args: SearchAndPrintArgs) {
    let mut command = args.command;
    let mut alias = args.alias;
    let mut tag = args.tag;
    let order_by_use = args.recent;
    let favourites_only = args.favourite;
    let print_style = args.print_style;
    let print_limit = args.display_limit;

    // If no search arguments are provided, generate a wizard to get them
    if display_search_args_wizard(&alias, &command, &tag) {
        let command_properties = match search_args_wizard() {
            Ok(properties) => properties,
            Err(e) => {
                error!(target: "Update Cmd", "Error setting command properties: {:?}", e);
                ErrorOutput::UserInput.print();
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
            alias,
            command,
            tag,
            order_by_use,
            favourites_only,
        },
        print_style,
        print_limit,
    ) {
        Ok(c) => c,
        Err(e) => match e {
            GetSelectedItemFromUserError::NoCommandsFound => {
                println!("\nNo commands found");
                return;
            }
            _ => {
                error!(target: "Update Cmd", "Failed to get selected command: {:?}", e);
                ErrorOutput::SelectCmd.print();
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
            error!(target: "Update Cmd", "Error setting command properties: {:?}", e);
            ErrorOutput::UserInput.print();
            return;
        }
    };

    let logic = Logic::try_default();
    if logic.is_err() {
        error!(target: "Update Cmd", "Failed to initialize logic: {:?}", logic.err());
        ErrorOutput::FailedToCommand("update".to_string()).print();
        return;
    }

    // Update the selected command
    match logic.as_ref().unwrap().handle_update_command(
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
            error!(target: "Update Cmd", "Failed to update command: {:?}", e);
            ErrorOutput::FailedToCommand("update".to_string()).print();
            return;
        }
    };

    println!("\nCommand updated successfully");
}
