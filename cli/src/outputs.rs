use data::models::InternalCommand;
use lazy_static::lazy_static;
use prettytable::{format, Attr, Cell, Row, Table};
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

/// Prints an command using the `prettytable` crate
pub fn print_internal_command_table(internal_command: &InternalCommand) {
    spacing();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    table.add_row(Row::new(vec![
        Cell::new("Command:").with_style(Attr::Bold),
        Cell::new(&internal_command.command),
    ]));
    if let Some(tag) = &internal_command.tag {
        table.add_row(Row::new(vec![
            Cell::new("Tag:").with_style(Attr::Bold),
            Cell::new(tag),
        ]));
    }
    if let Some(note) = &internal_command.note {
        table.add_row(Row::new(vec![
            Cell::new("Note:").with_style(Attr::Bold),
            Cell::new(note),
        ]));
    }
    let favourite_status = if internal_command.favourite {
        "Yes"
    } else {
        "No"
    };
    table.add_row(Row::new(vec![
        Cell::new("Favourite:").with_style(Attr::Bold),
        Cell::new(favourite_status),
    ]));

    table.printstd();
}

/// Printing vertical space
pub fn spacing() {
    println!();
}

pub enum Output<'a> {
    NoCommandsFound,
    UpdateCommandSectionTitle,
    UpdateCommandSuccess,
    AddCommandSuccess,
    DeleteCommandSuccess,
    ExportCommandsSuccess(&'a Path),
    ImportCommandsSuccess(u64, &'a Path),
    CommandCopiedToClipboard,
    ConfigCommandSuccess,
}

impl fmt::Display for Output<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Output::NoCommandsFound => "<bold>No commands found</bold>\n".to_string(),
            Output::UpdateCommandSectionTitle => "<section>Update Command:</section>".to_string(),
            Output::UpdateCommandSuccess => "✅ <bold>Command updated</bold>\n".to_string(),
            Output::AddCommandSuccess => "✅ <bold>Command added</bold>\n".to_string(),
            Output::DeleteCommandSuccess => "✅ <bold>Command deleted</bold>\n".to_string(),
            Output::ExportCommandsSuccess(file) => {
                format!("✅ <bold>Commands exported to {:?}</bold>\n", file)
            }
            Output::ImportCommandsSuccess(num_cmds, file) => {
                format!(
                    "✅ <bold>{} commands imported from {:?}</bold>\n",
                    num_cmds, file
                )
            }
            Output::CommandCopiedToClipboard => {
                "✅ <bold>Command copied to clipboard</bold>\n".to_string()
            }
            Output::ConfigCommandSuccess => "✅ <bold>Config updated</bold>\n".to_string(),
        };

        write!(f, "{}", format_output(&message))
    }
}

impl Output<'_> {
    pub fn print(&self) {
        spacing();
        println!("{}", self);
    }
}

pub enum ErrorOutput {
    UserInput,
    AddCommand,
    UpdateCommand,
    DeleteCommand,
    SearchCommand,
    Export,
    Import,
    Logger,
    LoadConfig,
    ConfigCommand,
}

impl fmt::Display for ErrorOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ErrorOutput::UserInput => "Failed to get input",
            ErrorOutput::AddCommand => "Failed to add command",
            ErrorOutput::UpdateCommand => "Failed to update command",
            ErrorOutput::DeleteCommand => "Failed to delete command",
            ErrorOutput::SearchCommand => "Failed to search command",
            ErrorOutput::Export => "Failed to export stack",
            ErrorOutput::Import => "Failed to import stack",
            ErrorOutput::Logger => "Failed to initialize the logger",
            ErrorOutput::LoadConfig => "Failed to load user config",
            ErrorOutput::ConfigCommand => "Failed to update config",
        };

        write!(
            f,
            "{}",
            format_output(&format!("❌ <bold>{}</bold>", message))
        )
    }
}

impl ErrorOutput {
    pub fn print(&self) {
        spacing();
        println!("{}", self);
        spacing();
    }
}
