use crate::{
    args::SearchAndPrintArgs,
    command::search_utils::{
        display_search_args_wizard, get_searched_commands, search_args_wizard,
        GetSelectedItemFromUserError,
    },
};
use log::error;
use logic::command::SearchCommandArgs;

/// UI handler for the delete command
pub fn handle_delete_command(args: SearchAndPrintArgs) {
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
                error!(target: "Delete Cmd", "Error setting command properties: {:?}", e);
                println!("Failed to get input");
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
                error!(target: "Delete Cmd", "Failed to get selected command: {:?}", e);
                println!("Failed to select command");
                return;
            }
        },
    };

    // Delete the selected command
    match logic::command::handle_delete_command(selected_command.id) {
        Ok(_) => {}
        Err(e) => {
            error!(target: "Delete Cmd", "Failed to delete command: {:?}", e);
            println!("Failed to delete selected command");
            return;
        }
    };

    println!("\nCommand deleted successfully");
}
