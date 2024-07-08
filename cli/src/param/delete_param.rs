//! Add a parameter to a command

use data::models::Command;

use super::param_utils::{select_parameters, ParamUtilError};

pub fn handle_delete_param_command(command: Command, print_limit: u32) {
    let params = match logic::param::get_params(command.id) {
        Ok(params) => params,
        Err(e) => {
            println!("Param Delete Cmd: Error getting parameters: {:?}", e);
            return;
        }
    };

    let param_to_delete = match select_parameters(&params, print_limit) {
        Ok(param) => param,
        Err(e) => match e {
            ParamUtilError::NoParams => {
                println!("\nSelected command does not have any parameters");
                return;
            }
            _ => {
                println!("Param Delete Cmd: Error listing parameters: {:?}", e);
                return;
            }
        },
    };

    match logic::param::delete_param(param_to_delete.id) {
        Ok(_) => println!("\nParameter deleted successfully"),
        Err(e) => println!("Param Delete Cmd: Error deleting parameter: {:?}", e),
    }
}
