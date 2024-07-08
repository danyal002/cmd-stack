//! Add a parameter to a command

use data::models::{Command, InternalParameter};
use inquire::{InquireError, Text};
use super::param_utils::{select_parameters, ParamUtilError};


fn update_param_wizard(cmd_id: u64, cur_symbol: String, cur_regex: String, cur_note: Option<String>) -> Result<InternalParameter, InquireError> {
    let symbol = Text::new("Symbol:")
        .with_initial_value(&cur_symbol)
        .prompt()?;

    let regex = Text::new("Regex:")
        .with_initial_value(&cur_regex)
        .prompt()?;

    let note = Text::new("Note:")
        .with_initial_value(&cur_note.unwrap_or(String::from("")))
        .prompt()?;

    return Ok(InternalParameter {
        command_id: cmd_id,
        symbol: symbol,
        regex: regex,
        note: if note != "" { Some(note) } else { None },
    });
}

pub fn handle_update_param_command(command: Command, print_limit: u32) {
    let params = match logic::param::get_params(command.id) {
        Ok(params) => params,
        Err(e) => {
            println!("Param Update Cmd: Error getting parameters: {:?}", e);
            return;
        }
    };

    let param_to_update = match select_parameters(&params, print_limit) {
        Ok(param) => param,
        Err(e) => match e {
            ParamUtilError::NoParams => {
                println!("\nSelected command does not have any parameters");
                return;
            }
            _ => {
                println!("Param Update Cmd: Error listing parameters: {:?}", e);
                return;
            }
        },
    };

    let updated_internal_params = match update_param_wizard(
        param_to_update.internal_parameter.command_id, 
        param_to_update.internal_parameter.symbol, 
        param_to_update.internal_parameter.regex, 
        param_to_update.internal_parameter.note
    ) {
        Ok(properties) => properties,
        Err(e) => {
            println!("Param Update Cmd: Error setting command properties: {:?}", e);
            return;
        }
    };

    match logic::param::update_param(param_to_update.id, updated_internal_params) {
        Ok(_) => {
            println!("Parameter updated successfully");
        }
        Err(e) => {
            println!("Param Update Cmd: Error updating parameter: {:?}", e);
        }
    }
}
