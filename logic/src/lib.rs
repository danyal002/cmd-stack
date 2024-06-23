//! # Logic
//!
//! This crate handles the business logic of the application

use std::error::Error;

pub fn handle_logic_request(request: LogicRequest) -> Result<(), Box<dyn Error>> {
    match request {
        LogicRequest::AddCommand(params) => {
            println!("Command: {}", params.command);
            println!("Alias: {:?}", params.alias);
            println!("Tag: {:?}", params.tag);
            println!("Note: {:?}", params.note);
        }
    }

    return Ok(());
}

pub enum LogicRequest {
    AddCommand(AddCommandParams),
}

pub struct AddCommandParams {
    pub command: String,
    pub alias: String,
    pub tag: Option<String>,
    pub note: Option<String>,
}
