//! Add a parameter to a command

use data::models::Command;

pub fn handle_update_param_command(command: Command) {
    println!("Update a parameter {:?}", command);
}
