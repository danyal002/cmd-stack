pub mod add_command;
pub mod config_command;
pub mod delete_command;
pub mod export_command;
pub mod import_command;
pub mod search_command;
pub mod search_utils;
pub mod update_command;

use inquire::{
    validator::{StringValidator, Validation},
    CustomUserError,
};

#[derive(Clone)]
pub struct CommandInputValidator;

impl StringValidator for CommandInputValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        if !input.trim().is_empty() {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Command must not be empty".into()))
        }
    }
}
