use crate::{
    args::SearchAndPrintArgs,
    command::search_utils::{
        copy_text, display_search_args_wizard, get_searched_commands, search_args_wizard,
        GetSelectedItemFromUserError,
    },
    outputs::ErrorOutput,
};
use log::error;
use logic::{command::SearchCommandArgs, Logic};

/// UI handler for the search command
pub fn handle_search_commands(args: SearchAndPrintArgs) {
    let mut command = args.command;
    let mut alias = args.alias;
    let mut tag = args.tag;
    let order_by_use = args.recent;
    let favourites_only = args.favourite;
    let skip_prompts = args.skip_prompts;
    let print_style = args.print_style;
    let print_limit = args.display_limit;

    // If no search arguments are provided, generate a wizard to get them
    if display_search_args_wizard(&alias, &command, &tag, skip_prompts) {
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
                error!(target: "Search Cmd", "Failed to get selected command: {:?}", e);
                ErrorOutput::SelectCmd.print();
                return;
            }
        },
    };

    let logic = Logic::try_default();
    if logic.is_err() {
        error!(
            target: "Search Cmd", "Failed to initialize logic: {:?}",
            logic.err()
        );
        ErrorOutput::GenerateParam.print();
        return;
    }

    let copied_text = match logic
        .as_ref()
        .unwrap()
        .handle_generate_param(selected_command.clone())
    {
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
    copy_text("Search Cmd", copied_text);

    match logic
        .unwrap()
        .handle_update_command_last_used_prop(selected_command.id)
    {
        Ok(_) => {}
        Err(e) => {
            // Does not matter to the user if this does not work
            error!(
                target: "Search Cmd", "Failed to update command last used prop: {:?}",
                e
            );
        }
    };
}
