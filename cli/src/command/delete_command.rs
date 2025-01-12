use crate::{
    args::SearchAndPrintArgs,
    command::search_utils::{
        display_search_args_wizard, get_searched_commands, search_args_wizard,
        GetSelectedItemFromUserError,
    },
    outputs::{ErrorOutput, Output},
};
use log::error;
use logic::{command::SearchCommandArgs, Logic};

/// UI handler for the delete command
pub fn handle_delete_command(args: SearchAndPrintArgs) {
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
                error!(target: "Delete Cmd", "Error setting command properties: {:?}", e);
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
                Output::NoCommandsFound.print();
                return;
            }
            _ => {
                error!(target: "Delete Cmd", "Failed to get selected command: {:?}", e);
                ErrorOutput::SelectCmd.print();
                return;
            }
        },
    };

    let logic = Logic::try_default();
    if logic.is_err() {
        error!(target: "Delete Cmd", "Failed to initialize logic: {:?}", logic.err());
        ErrorOutput::FailedToCommand("delete".to_string()).print();
        return;
    }

    // Delete the selected command
    match logic
        .as_ref()
        .unwrap()
        .handle_delete_command(selected_command.id)
    {
        Ok(_) => {}
        Err(e) => {
            error!(target: "Delete Cmd", "Failed to delete command: {:?}", e);
            ErrorOutput::FailedToCommand("delete".to_string()).print();
            return;
        }
    };

    Output::DeleteCommandSuccess.print();
}
