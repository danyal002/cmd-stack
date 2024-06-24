pub struct Command {
    pub id: u64,
    pub alias: String,
    pub command: String,
    pub tag: Option<String>,
    pub note: Option<String>,
    pub last_used: u64,
}

pub struct Parameter {
    pub id: u64,
    pub command_id: u64,
    pub name: String,
    pub symbol: String,
    pub default_value: Option<String>,
    pub note: Option<String>,
}