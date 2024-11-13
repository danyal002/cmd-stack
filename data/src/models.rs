use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Represents the properties of a command that the user will
/// have knowledge about
pub struct InternalCommand {
    pub alias: String,
    pub command: String,
    pub tag: Option<String>,
    pub note: Option<String>,
    pub favourite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Stores all properties of a command in the database
pub struct Command {
    pub id: i64,
    pub last_used: i64,
    pub internal_command: InternalCommand,
}
