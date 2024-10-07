use crate::args::ParamCommands;
use crate::command::search_utils::{
    display_search_args_wizard, get_searched_commands, search_args_wizard,
    GetSelectedItemFromUserError,
};
use crate::outputs::ErrorOutput;
use log::error;
use logic::command::SearchCommandArgs;
use logic::new_logic;

mod add_param;
mod delete_param;
mod list_param;
mod param_utils;
mod update_param;

pub fn handle_param_command(param_command: ParamCommands) {
    let param_args = match &param_command {
        ParamCommands::List(list_param_args) => list_param_args,
        ParamCommands::Add(add_param_args) => add_param_args,
        ParamCommands::Update(update_param_args) => update_param_args,
        ParamCommands::Delete(delete_param_args) => delete_param_args,
    };

    let mut command = param_args.command.clone();
    let mut alias = param_args.alias.clone();
    let mut tag = param_args.tag.clone();
    let print_style = param_args.print_style.clone();
    let print_limit = param_args.display_limit;

    // If no search arguments are provided, generate a wizard to get them
    if display_search_args_wizard(&alias, &command, &tag) {
        let command_properties = match search_args_wizard() {
            Ok(properties) => properties,
            Err(e) => {
                error!(target: "Param Cmd", "Error setting command properties: {:?}", e);
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
                error!(target: "Param Cmd", "Failed to get selected command: {:?}", e);
                ErrorOutput::SelectCmd.print();
                return;
            }
        },
    };

    let logic = new_logic();
    if logic.is_err() {
        error!(target: "Param Cmd", "Error getting parameters: {:?}", logic.err());
        ErrorOutput::FailedToCommand("get parameters".to_string()).print();
        return;
    }

    // Get the parameters for the selected command
    let params = match logic.as_ref().unwrap().get_params(selected_command.id) {
        Ok(params) => params,
        Err(e) => {
            error!(target: "Param Cmd", "Error getting parameters: {:?}", e);
            ErrorOutput::FailedToCommand("get parameters".to_string()).print();
            return;
        }
    };

    match param_command {
        ParamCommands::List(_) => list_param::handle_list_param_command(params, print_limit),
        ParamCommands::Add(_) => add_param::handle_add_param_command(selected_command),
        ParamCommands::Update(_) => update_param::handle_update_param_command(params, print_limit),
        ParamCommands::Delete(_) => delete_param::handle_delete_param_command(params, print_limit),
    }
}
