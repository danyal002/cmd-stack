#[derive(Debug, Clone)]
pub struct InternalCommand {
    pub alias: String,
    pub command: String,
    pub tag: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub id: u64,
    pub last_used: u64,
    pub internal_command: InternalCommand,
}

#[derive(Debug, Clone)]
pub struct InternalParameter {
    pub name: String,
    pub symbol: String,
    pub default_value: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub id: u64,
    pub command_id: u64,
    pub internal_parameter: InternalParameter,
}
