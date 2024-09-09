pub enum ErrorOutput {
    UserInput,
    SelectCmd,
    SelectParam,
    FailedToCommand(String),
    FailedToParam(String),
    FailedToCopy(String),
    Export,
    NotJson,
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
            ErrorOutput::FailedToCopy(cmd) => {
                println!("Failed to add the selected command to clipboard. Please copy the following text:");
                println!();
                println!("{}\n", cmd);
            }
            ErrorOutput::Export => println!("Failed to export commands"),
            ErrorOutput::NotJson => {
                println!("Failed to export because provided file is not a JSON")
            }
            ErrorOutput::Import => println!("Failed to import commands"),
            ErrorOutput::AddCmd => println!("Failed to add command"),
            ErrorOutput::GenerateParam => println!("Failed to generate parameter"),
            ErrorOutput::AddParams => println!("Failed to add parameters"),
            ErrorOutput::ListParams => println!("Failed to list parameters"),
            ErrorOutput::Logger => println!("Failed to initialize logger"),
        }
    }
}
