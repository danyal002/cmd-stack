//! Add a parameter to a command

use data::models::Parameter;
use log::error;

use super::param_utils::{select_parameters, ParamUtilError};

pub fn handle_delete_param_command(params: Vec<Parameter>, print_limit: u32) {
    let param_to_delete = match select_parameters(&params, print_limit) {
        Ok(param) => param,
        Err(e) => match e {
            ParamUtilError::NoParams => {
                println!("\nSelected command does not have any parameters");
                return;
            }
            _ => {
                error!(target: "Param Delete Cmd", "Error listing parameters: {:?}", e);
                println!("Failed to select parameter");
                return;
            }
        },
    };

    match logic::param::delete_param(param_to_delete.id) {
        Ok(_) => println!("\nParameter deleted successfully"),
        Err(e) => {
            error!(target: "Param Delete Cmd", "Error deleting parameter: {:?}", e);
            println!("Failed to delete selected parameter");
        }
    }
}
