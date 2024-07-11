//! Add a parameter to a command

use crate::param::param_utils::{list_parameters, ParamUtilError};
use data::models::Parameter;
use log::error;

pub fn handle_list_param_command(params: Vec<Parameter>, print_limit: u32) {
    match list_parameters(params, print_limit) {
        Ok(_) => {}
        Err(e) => match e {
            ParamUtilError::NoParams => {
                println!("\nSelected command does not have any parameters");
            }
            _ => {
                error!(target: "Param List Cmd", "Error listing parameters: {:?}", e);
                println!("Failed to list parameters");
            }
        },
    }
}
