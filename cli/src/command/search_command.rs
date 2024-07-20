use crate::{
    args::SearchAndPrintArgs,
    command::search_utils::{
        display_search_args_wizard, get_searched_commands, search_args_wizard,
        GetSelectedItemFromUserError,
    },
    outputs::ErrorOutput,
};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use log::error;
use logic::command::{handle_update_command_last_used_prop, SearchCommandArgs};

/// UI handler for the search command
pub fn handle_search_commands(args: SearchAndPrintArgs) {
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
                error!(target: "Search Cmd", "Error setting command properties: {:?}", e);
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
                println!("\nNo commands found");
                return;
            }
            _ => {
                error!(target: "Search Cmd", "Failed to get selected command: {:?}", e);
                ErrorOutput::SelectCmd.print();
                return;
            }
        },
    };

    let copied_text = match logic::param::handle_generate_param(selected_command.clone()) {
        Ok(c) => c,
        Err(e) => {
            error!(target: "Search Cmd",
                "Search Cmd: Failed to generate parameters for selected command: {:?}",
                e
            );
            ErrorOutput::GenerateParam.print();
            return;
        }
    };

    // Copy the selected command to the clipboard
    let mut clipboard = match ClipboardContext::new() {
        Ok(ctx) => ctx,
        Err(e) => {
            error!(target: "Search Cmd", "Failed to initialize the clipboard: {:?}", e);
            ErrorOutput::FailedToCommand("copy".to_string()).print();
            return;
        }
    };
    match clipboard.set_contents(copied_text.clone()) {
        Ok(()) => println!("\nCommand copied to clipboard: {}", copied_text),
        Err(e) => {
            error!(target: "Search Cmd", "Failed copy command to clipboard: {:?}", e);
            ErrorOutput::FailedToCommand("copy".to_string()).print();
            return;
        }
    }

    match handle_update_command_last_used_prop(selected_command.id) {
        Ok(_) => {}
        Err(e) => {
            // Does not matter to the user if this does not work
            error!(
                target: "Search Cmd", "Failed to update command last used prop: {:?}",
                e
            );
            return;
        }
    };
}
