use data::models::{Command, InternalParameter};
use inquire::{validator::Validation, InquireError};
use log::error;
use logic::new_logic;
use rand_regex::Regex;

use crate::outputs::ErrorOutput;

/// Generates a wizard to set the properties of a parameter
fn set_param_properties_wizard(command_id: i64) -> Result<InternalParameter, InquireError> {
    println!("\nSet the properties of the parameter");
    let symbol = inquire::Text::new("Symbol:").prompt()?;

    // Validate that the given regex is valid
    let validator = |input: &str| {
        let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
        let hir = match parser.parse(input) {
            Ok(hir) => hir,
            Err(_) => {
                return Ok(Validation::Invalid("Your regex is invalid".into()));
            }
        };

        match Regex::with_hir(hir, 100) {
            Ok(_) => Ok(Validation::Valid),
            Err(_) => Ok(Validation::Invalid("Your regex is invalid".into())),
        }
    };
    let regex = inquire::Text::new("Regex:")
        .with_validator(validator)
        .prompt()?;

    let note = inquire::Text::new("Note (Optional):").prompt()?;

    Ok(InternalParameter {
        command_id,
        symbol,
        regex,
        note: if !note.is_empty() { Some(note) } else { None },
    })
}

fn get_params_from_user(command_id: i64) -> Result<Vec<InternalParameter>, InquireError> {
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

    Ok(params)
}

/// UI handler for add parameter command
pub fn handle_add_param_command(command: Command) {
    let params = match get_params_from_user(command.id) {
        Ok(p) => p,
        Err(e) => {
            error!(target: "Add Param Cmd", "Error setting parameter properties: {:?}", e);
            ErrorOutput::UserInput.print();
            return;
        }
    };

    let logic = new_logic();
    if logic.is_err() {
        error!(target: "Add Param Cmd", "Failed to initialize logic: {:?}", logic.err());
        ErrorOutput::AddParams.print();
        return;
    }

    match logic.as_ref().unwrap().handle_add_param(params) {
        Ok(_) => println!("\nParameters added successfully"),
        Err(e) => {
            error!(target: "Add Param Cmd", "Error adding parameters: {:?}", e);
            ErrorOutput::AddParams.print();
        }
    }
}
