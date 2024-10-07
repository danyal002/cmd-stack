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

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the properties of a parameter that the user will
/// have knowledge about
pub struct InternalParameter {
    pub command_id: i64,
    pub symbol: String,
    pub regex: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone)]
/// Stores all properties of a parameter in the database
pub struct Parameter {
    pub id: i64,
    pub internal_parameter: InternalParameter,
}
