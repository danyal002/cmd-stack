use crate::{args::SearchAndPrintArgs, search_utils::{get_selected_item_from_user, search_args_wizard, GetSelectedItemFromUserArgs}};
use logic::command::SearchCommandArgs;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub fn handle_search_command(args: SearchAndPrintArgs) {
    let mut command = args.command;
    let mut alias = args.alias;
    let mut tag = args.tag;
    let print_style = args.print_style;
    let print_limit = args.display_limit;

    if alias.is_none() && tag.is_none() && command.is_none() {
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

    let selected_command = match get_selected_item_from_user(GetSelectedItemFromUserArgs {
        search_args: SearchCommandArgs {
            alias: alias,
            command: command,
            tag: tag,
        },
        print_style: print_style,
        display_limit: print_limit,
    }) {
        Ok(c) => c,
        Err(e) => {
            println!("Search: Failed to get selected command: {:?}", e);
            return;
        }
    };

    let mut clipboard = ClipboardContext::new().unwrap();
    clipboard.set_contents(selected_command.command.clone()).unwrap();

    println!("Command copied to clipboard: {}", selected_command.command)
}
