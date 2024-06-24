use crate::{
    args::SearchAndPrintArgs,
    search_utils::{get_searched_commands, search_args_wizard},
};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use logic::command::SearchCommandArgs;

pub fn handle_search_commands(args: SearchAndPrintArgs) {
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
        Err(e) => {
            println!("Search Cmd: Failed to get selected command: {:?}", e);
            return;
        }
    };

    let mut clipboard = ClipboardContext::new().unwrap();
    clipboard
        .set_contents(selected_command.command.clone())
        .unwrap();

    println!("Command copied to clipboard: {}", selected_command.command)
}
