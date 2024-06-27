//! Add a parameter to a command

use data::models::{Command, InternalParameter};
use inquire::InquireError;

/// Generates a wizard to set the properties of a parameter
fn set_param_properties_wizard(command_id: u64) -> Result<InternalParameter, InquireError> {
    println!("\nSet the properties of the parameter");
    let symbol = inquire::Text::new("Symbol:").prompt()?;
    let regex = inquire::Text::new("Regex:").prompt()?;
    let note = inquire::Text::new("Note (Optional):").prompt()?;

    return Ok(InternalParameter {
        command_id: command_id,
        symbol: symbol,
        regex: regex,
        note: if note != "" { Some(note) } else { None },
    });
}

fn get_params_from_user(command_id: u64) -> Result<Vec<InternalParameter>, InquireError> {
    let mut params = Vec::new();
    loop {
        let param = set_param_properties_wizard(command_id)?;
        params.push(param);

        let add_another =
            inquire::Select::new("Add another parameter?", vec!["Yes", "No"]).prompt()?;

        if add_another == "No" {
            break;
        }
    }

    return Ok(params);
}

pub fn handle_add_param_command(command: Command) {
    let params = match get_params_from_user(command.id) {
        Ok(p) => p,
        Err(e) => {
            println!("Add Param Cmd: Error setting parameter properties: {:?}", e);
            return;
        }
    };

    match logic::param::handle_add_param(params) {
        Ok(_) => println!("Parameters added successfully"),
        Err(e) => println!("Add Param Cmd: Error adding parameters: {:?}", e),
    }
}
