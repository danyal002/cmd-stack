//! Add a parameter to a command

use crate::param::param_list::{list_parameters, ListParamError};
use data::models::Command;

pub fn handle_list_param_command(command: Command) {
    let params = match logic::param::get_params(command.id) {
        Ok(params) => params,
        Err(e) => {
            println!("Error getting parameters: {:?}", e);
            return;
        }
    };

    match list_parameters(params) {
        Ok(_) => {}
        Err(e) => match e {
            ListParamError::NoParams => {
                println!("\nSelected command does not have any parameters");
            }
            _ => {
                println!("\nError listing parameters: {:?}", e);
            }
        },
    }
}
