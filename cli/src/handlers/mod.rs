pub mod add;
pub mod cli_prompter;
pub mod config;
pub mod delete;
pub mod export;
pub mod import;
pub mod search;
pub mod update;

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
