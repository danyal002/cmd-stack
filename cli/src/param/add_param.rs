//! Add a parameter to a command

use data::models::{Command, InternalParameter};
use inquire::{InquireError, validator::Validation};
use rand_regex::Regex;

/// Generates a wizard to set the properties of a parameter
fn set_param_properties_wizard(command_id: u64) -> Result<InternalParameter, InquireError> {
    println!("\nSet the properties of the parameter");
    let symbol = inquire::Text::new("Symbol:").prompt()?;
    
    // Validate that the given regex is valid
    let validator = |input: &str| {
        let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
        let hir = parser.parse(input);
        if hir.is_err() {
            return Ok(Validation::Invalid("Your regex is invalid".into()));
        }

        match Regex::with_hir(hir.unwrap(), 100) {
            Ok(_) => Ok(Validation::Valid),
            Err(_) => Ok(Validation::Invalid(
                "Your regex is invalid".into(),
            )),
        }
    };
    let regex = inquire::Text::new("Regex:")
        .with_validator(validator)
        .prompt()?;
    
    
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
