use crate::{args::ListArgs, search_utils::{get_listed_commands, GetSelectedItemFromUserError}};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use logic::command::handle_update_command_last_used_prop;

/// UI handler for the list command
pub fn handle_list_commands(args: ListArgs) {
    let recent = args.recent;
    let print_style = args.print_style;
    let print_limit = args.display_limit;
    let favourite = args.favourite;

    // Get the selected command
    let selected_command = match get_listed_commands(recent, favourite, print_style, print_limit) {
        Ok(c) => c,
        Err(e) => match e {
            GetSelectedItemFromUserError::NoCommandsFound => {
                println!("No commands found");
                return;
            }
            _ => {
                println!("List Cmd: Failed to get selected command: {:?}", e);
                return;
            }
        }
    };

    // Copy the selected command to the clipboard
    let mut clipboard = ClipboardContext::new().unwrap();
    clipboard
        .set_contents(selected_command.internal_command.command.clone())
        .unwrap();

    println!(
        "Command copied to clipboard: {}",
        selected_command.internal_command.command
    );

    match handle_update_command_last_used_prop(selected_command.id) {
        Ok(_) => {}
        Err(e) => {
            println!("List Cmd: Failed to update command last used prop: {:?}", e);
            return;
        }
    };
}
