use crate::{
    args::SearchAndPrintArgs,
    search_utils::{
        display_search_args_wizard, get_searched_commands, search_args_wizard,
        GetSelectedItemFromUserError,
    },
};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use logic::{command::SearchCommandArgs, Logic};

/// UI handler for the search command
pub fn handle_search_commands(logic_layer: Logic, args: SearchAndPrintArgs) {
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
        &logic_layer,
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
                println!("Search Cmd: Failed to get selected command: {:?}", e);
                return;
            }
        },
    };

    let copied_text = match logic_layer.handle_generate_param(selected_command.clone()) {
        Ok(c) => c,
        Err(e) => {
            println!(
                "Search Cmd: Failed to generate parameters for selected command: {:?}",
                e
            );
            return;
        }
    };

    // Copy the selected command to the clipboard
    let mut clipboard = ClipboardContext::new().unwrap();
    clipboard.set_contents(copied_text).unwrap();

    println!(
        "\nCommand copied to clipboard: {}",
        selected_command.internal_command.command
    );

    match logic_layer.handle_update_command_last_used_prop(selected_command.id) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "Search Cmd: Failed to update command last used prop: {:?}",
                e
            );
            return;
        }
    };
}
