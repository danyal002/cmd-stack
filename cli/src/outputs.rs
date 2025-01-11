use std::collections::HashMap;

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

/// Converts the given coded text into text for output to the CLI
pub fn format_output(text: &str) -> String {
    let replacements = HashMap::from([
        ("<bold>", "\x1b[1m"),
        ("</bold>", "\x1b[22m"),
        ("<italics>", "\x1b[3m"),
        ("</italics>", "\x1b[23m"),
    ]);

    replacements
        .into_iter()
        .fold(text.to_string(), |accumulator, (key, replacement_val)| {
            accumulator.replace(key, replacement_val)
        })
}
