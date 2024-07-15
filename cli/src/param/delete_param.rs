use data::models::Parameter;
use log::error;

use crate::outputs::ErrorOutput;

use super::param_utils::{select_parameters, ParamUtilError};

/// UI handler for delete parameter command
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
                ErrorOutput::SelectParam.print();
                return;
            }
        },
    };

    match logic::param::delete_param(param_to_delete.id) {
        Ok(_) => println!("\nParameter deleted successfully"),
        Err(e) => {
            error!(target: "Param Delete Cmd", "Error deleting parameter: {:?}", e);
            ErrorOutput::FailedToParam("delete".to_string()).print();
        }
    }
}
