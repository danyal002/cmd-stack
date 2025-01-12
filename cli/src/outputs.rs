use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

lazy_static! {
    /// To see how colours are rendered refer to this Wikipedia page:
    ///
    /// https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit
    static ref MACRO_REPLACEMENTS: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("<bold>", "\x1b[1m"),                // Bold
            ("</bold>", "\x1b[22m"),              // Unbold
            ("<italics>", "\x1b[3m"),             // Italicize
            ("</italics>", "\x1b[23m"),           // Un-italicize
            ("<section>", "\x1b[1m\x1b[4m"),      // Bold + Underline
            ("</section>", "\x1b[22m\x1b[24m"),   // Unbold + remove underline
            ("<success>", "\x1b[32m"),            // Green
            ("</success>", "\x1b[39m"),           // Remove color
            ("<error>", "\x1b[31m\x1b[1m"),       // Red + Bold
            ("</error>", "\x1b[39m\x1b[22m"),     // Remove bold + color
        ])
    };
}

/// Converts the given coded text into ANSI escape codes for printing to the CLI:
///
/// https://en.wikipedia.org/wiki/ANSI_escape_code
pub fn format_output(text: &str) -> String {
    MACRO_REPLACEMENTS
        .iter()
        .fold(text.to_string(), |acc, (key, val)| acc.replace(key, val))
}

pub enum Output<'a> {
    NoCommandsFound,
    UpdateCommandSectionTitle,
    UpdateCommandSuccess,
    AddCommandSuccess,
    DeleteCommandSuccess,
    ExportCommandsSuccess(&'a Path),
    ImportCommandsSuccess(u32, &'a Path),
    CommandCopiedToClipboard(String),
}

impl fmt::Display for Output<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Output::NoCommandsFound => "No commands found".to_string(),
            Output::UpdateCommandSectionTitle => "<section>Update Command:</section>".to_string(),
            Output::UpdateCommandSuccess => {
                "<success>Command updated successfully</success>".to_string()
            }
            Output::AddCommandSuccess => {
                "<success>Command added successfully</success>".to_string()
            }
            Output::DeleteCommandSuccess => {
                "<success>Command deleted successfully</success>".to_string()
            }
            Output::ExportCommandsSuccess(file) => {
                format!(
                    "<success>Commands exported successfully to {:?}</success>",
                    file
                )
            }
            Output::ImportCommandsSuccess(num_cmds, file) => {
                format!(
                    "<success>{} commands imported successfully from {:?}</success>",
                    num_cmds, file
                )
            }
            Output::CommandCopiedToClipboard(cmd) => {
                format!(
                    "<success><bold>Command copied to clipboard:</bold></success> {}",
                    cmd
                )
            }
        };

        write!(f, "{}", format_output(&message))
    }
}

impl Output<'_> {
    pub fn print(&self) {
        println!(); // Spacing
        println!("{}", self);
    }
}

pub enum ErrorOutput {
    UserInput,
    SelectCmd,
    FailedToCommand(String),
    FailedToCopy(String),
    Export,
    NotJson,
    Import,
    AddCommand,
    GenerateParam,
    Logger,
}

impl fmt::Display for ErrorOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ErrorOutput::UserInput => "Failed to get input",
            ErrorOutput::SelectCmd => "Failed to select command",
            ErrorOutput::FailedToCommand(op) => &format!("Failed to {} selected command", op),
            ErrorOutput::FailedToCopy(cmd) => &format!("Failed to add the selected command to clipboard. Please copy the following text:\n\n{}\n", cmd),
            ErrorOutput::Export => "Failed to export commands",
            ErrorOutput::NotJson => "Failed to export because provided file is not a JSON",
            ErrorOutput::Import => "Failed to import commands",
            ErrorOutput::AddCommand => "Failed to add command",
            ErrorOutput::GenerateParam => "Failed to generate parameter",
            ErrorOutput::Logger => "Failed to initialize logger",
        };

        write!(
            f,
            "{}",
            format_output(&format!("<error>{}</error>", message))
        )
    }
}

impl ErrorOutput {
    pub fn print(&self) {
        println!(); // Spacing
        println!("{}", self);
    }
}
