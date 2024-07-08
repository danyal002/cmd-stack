//! Add a parameter to a command

use crate::param::param_utils::{list_parameters, ParamUtilError};
use data::models::Command;

pub fn handle_list_param_command(command: Command, print_limit: u32) {
    let params = match logic::param::get_params(command.id) {
        Ok(params) => params,
        Err(e) => {
            println!("Param List Cmd: Error getting parameters: {:?}", e);
            return;
        }
    };

    match list_parameters(params, print_limit) {
        Ok(_) => {}
        Err(e) => match e {
            ParamUtilError::NoParams => {
                println!("\nSelected command does not have any parameters");
            }
            _ => {
                println!("Param List Cmd: Error listing parameters: {:?}", e);
            }
        },
    }
}
