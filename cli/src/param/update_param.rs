//! Add a parameter to a command

use crate::outputs::ErrorOutput;

use super::param_utils::{select_parameters, ParamUtilError};
use data::models::{InternalParameter, Parameter};
use inquire::{InquireError, Text};
use log::error;

fn update_param_wizard(
    command_id: i64,
    cur_symbol: String,
    cur_regex: String,
    cur_note: Option<String>,
) -> Result<InternalParameter, InquireError> {
    println!("\nSet the properties of the parameter");
    let symbol = Text::new("Symbol:")
        .with_initial_value(&cur_symbol)
        .prompt()?;

    let regex = Text::new("Regex:")
        .with_initial_value(&cur_regex)
        .prompt()?;

    let note = Text::new("Note:")
        .with_initial_value(&cur_note.unwrap_or(String::from("")))
        .prompt()?;

    Ok(InternalParameter {
        command_id,
        symbol,
        regex,
        note: if !note.is_empty() { Some(note) } else { None },
    })
}

/// UI handler for update parameter command
pub fn handle_update_param_command(params: Vec<Parameter>, print_limit: u32) {
    let param_to_update = match select_parameters(&params, print_limit) {
        Ok(param) => param,
        Err(e) => match e {
            ParamUtilError::NoParams => {
                println!("\nSelected command does not have any parameters");
                return;
            }
            _ => {
                error!(target: "Param Update Cmd", "Error listing parameters: {:?}", e);
                ErrorOutput::SelectParam.print();
                return;
            }
        },
    };

    let updated_internal_params = match update_param_wizard(
        param_to_update.internal_parameter.command_id,
        param_to_update.internal_parameter.symbol,
        param_to_update.internal_parameter.regex,
        param_to_update.internal_parameter.note,
    ) {
        Ok(properties) => properties,
        Err(e) => {
            error!(
                target: "Param Update Cmd", "Error setting command properties: {:?}", e
            );
            ErrorOutput::UserInput.print();
            return;
        }
    };

    match logic::param::update_param(param_to_update.id, updated_internal_params) {
        Ok(_) => {
            println!("\nParameter updated successfully");
        }
        Err(e) => {
            error!(target: "Param Update Cmd", "Error updating parameter: {:?}", e);
            ErrorOutput::FailedToParam("update".to_string()).print();
        }
    }
}
