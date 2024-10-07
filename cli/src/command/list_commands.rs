use crate::{
    args::ListArgs,
    command::search_utils::{copy_text, get_listed_commands, GetSelectedItemFromUserError},
    outputs::ErrorOutput,
};
use log::error;
use logic::new_logic;

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
                println!("\nNo commands found");
                return;
            }
            GetSelectedItemFromUserError::InquireError(ie) => match ie {
                inquire::InquireError::OperationInterrupted => {
                    // If the user cancelled the search, don't display anything
                    return;
                }
                _ => {
                    error!(target: "List Cmd", "Failed to get selected command: {:?}", ie);
                    ErrorOutput::SelectCmd.print();
                    return;
                }
            },
            _ => {
                error!(target: "List Cmd", "Failed to get selected command: {:?}", e);
                ErrorOutput::SelectCmd.print();
                return;
            }
        },
    };

    // Copy the selected command to the clipboard
    copy_text(
        "List Cmd",
        selected_command.internal_command.command.clone(),
    );

    let logic = new_logic();
    if logic.is_err() {
        error!(target: "List Cmd", "Failed to update command last used prop: {:?}", logic.err());
        return;
    }

    match logic
        .unwrap()
        .handle_update_command_last_used_prop(selected_command.id)
    {
        Ok(_) => {}
        Err(e) => {
            // Does not matter much to the user if this does not work
            error!(target: "List Cmd", "Failed to update command last used prop: {:?}", e);
        }
    };
}
