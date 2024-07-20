pub enum ErrorOutput {
    UserInput,
    SelectCmd,
    SelectParam,
    FailedToCommand(String),
    FailedToParam(String),
    Export,
    Import,
    AddCmd,
    GenerateParam,
    AddParams,
    ListParams,
    Logger,
}

impl ErrorOutput {
    pub fn print(&self) {
        println!(); // Spacing
        match self {
            ErrorOutput::UserInput => println!("Failed to get input"),
            ErrorOutput::SelectCmd => println!("Failed to select command"),
            ErrorOutput::SelectParam => println!("Failed to select parameter"),
            ErrorOutput::FailedToCommand(op) => println!("Failed to {} selected command", op),
            ErrorOutput::FailedToParam(op) => println!("Failed to {} selected parameter", op),
            ErrorOutput::Export => println!("Failed to export commands"),
            ErrorOutput::Import => println!("Failed to import commands"),
            ErrorOutput::AddCmd => println!("Failed to add command"),
            ErrorOutput::GenerateParam => println!("Failed to generate parameter"),
            ErrorOutput::AddParams => println!("Failed to add parameters"),
            ErrorOutput::ListParams => println!("Failed to list parameters"),
            ErrorOutput::Logger => println!("Failed to initialize logger"),
        }
    }
}
