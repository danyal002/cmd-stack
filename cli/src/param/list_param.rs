use crate::{
    outputs::ErrorOutput,
    param::param_utils::{list_parameters, ParamUtilError},
};
use data::models::Parameter;
use log::error;

/// UI handler for list parameter command
pub fn handle_list_param_command(params: Vec<Parameter>, print_limit: u32) {
    match list_parameters(params, print_limit) {
        Ok(_) => {}
        Err(e) => match e {
            ParamUtilError::NoParams => {
                println!("\nSelected command does not have any parameters");
            }
            ParamUtilError::GetSelectedItemFromUserError(ie) => {
                match ie {
                    inquire::InquireError::OperationInterrupted => {
                        // If the user cancelled the search, don't display anything
                    }
                    _ => {
                        error!(target: "Param List Cmd", "Error listing parameters: {:?}", ie);
                        ErrorOutput::ListParams.print();
                    }
                }
            }
        },
    }
}
