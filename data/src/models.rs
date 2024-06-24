#[derive(Debug, Clone)]
/// Represents the properties of a command that the user will 
/// have knowledge about 
pub struct InternalCommand {
    pub alias: String,
    pub command: String,
    pub tag: Option<String>,
    pub note: Option<String>,
    pub favourite: bool,
}

#[derive(Debug, Clone)]
/// Stores all properties of a command in the database
pub struct Command {
    pub id: u64,
    pub last_used: u64,
    pub internal_command: InternalCommand,
}

#[derive(Debug, Clone)]
/// Represents the properties of a parameter that the user will 
/// have knowledge about 
pub struct InternalParameter {
    pub name: String,
    pub symbol: String,
    pub default_value: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone)]
/// Stores all properties of a parameter in the database
pub struct Parameter {
    pub id: u64,
    pub command_id: u64,
    pub internal_parameter: InternalParameter,
}
