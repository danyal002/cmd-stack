use crate::{args::ListArgs, search_utils::get_listed_commands};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub fn handle_list_commands(args: ListArgs) {
    let recent = args.recent;
    let print_style = args.print_style;
    let print_limit = args.display_limit;
    let favourite = args.favourite;

    let selected_command = match get_listed_commands(recent, favourite, print_style, print_limit) {
        Ok(c) => c,
        Err(e) => {
            println!("List Cmd: Failed to get commands: {:?}", e);
            return;
        }
    };

    let mut clipboard = ClipboardContext::new().unwrap();
    clipboard
        .set_contents(selected_command.command.clone())
        .unwrap();

    println!("Command copied to clipboard: {}", selected_command.command)
}
