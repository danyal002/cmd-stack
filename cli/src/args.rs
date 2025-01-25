use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct CmdStackArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a command to your stack
    Add(AddArgs),

    /// Update a command in your stack
    Update(SearchAndPrintArgs),

    /// Delete a command in your stack
    Delete(SearchAndPrintArgs),

    /// Search for a command in your stack
    Search(SearchAndPrintArgs),

    /// Export stack to a JSON file
    Export(ImportExportArgs),

    /// Import stack from a JSON file
    Import(ImportExportArgs),

    /// Update the CLI config
    Config(ConfigArgs),
}

/// Arguments for adding a command
#[derive(Debug, Args)]
pub struct AddArgs {
    /// The command to add to your stack
    pub command: Option<String>,

    /// Notes relating to the command
    #[clap(long = "note", short = 'n')]
    pub note: Option<String>,

    /// The tag for the command
    #[clap(long = "tag", short = 't')]
    pub tag: Option<String>,

    /// Mark the command as favourite
    #[clap(long = "favourite", short = 'f', action)]
    pub favourite: bool,
}

/// Different supported printing styles for commands
#[derive(Debug, ValueEnum, Clone, Serialize, Deserialize)]
pub enum PrintStyle {
    /// Display the command, tag, and notes
    All,

    /// Only display the command
    Command,
}

impl fmt::Display for PrintStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrintStyle::All => write!(f, "All"),
            PrintStyle::Command => write!(f, "Command only"),
        }
    }
}

/// Arguments for searching and printing commands
#[derive(Debug, Args, Clone)]
pub struct SearchAndPrintArgs {
    /// The text used to filter by command when searching
    pub command: Option<String>,

    /// The text used to filter by tag when searching
    #[clap(long = "tag", short = 't')]
    pub tag: Option<String>,

    /// Display commands in order of most recent use
    #[clap(long = "recent", short = 'r', action)]
    pub recent: bool,

    /// Only display favourite commands
    #[clap(long = "favourite", short = 'f', action)]
    pub favourite: bool,

    /// Configure how commands are displayed
    #[clap(long = "print-style", short = 'p', value_enum)]
    pub print_style: Option<PrintStyle>,

    /// Configure how many commands are displayed at a time
    #[clap(long = "display-limit", short = 'd')]
    pub display_limit: Option<i32>,
}

/// Arguments for importing/exporting commands
#[derive(Debug, Args)]
pub struct ImportExportArgs {
    /// The relative path of the file
    pub file: String,
}

/// Arguments for setting the CLI config
#[derive(Debug, Args, Clone)]
pub struct ConfigArgs {
    /// Configure how commands are displayed
    #[clap(long = "print-style", short = 'p', value_enum)]
    pub print_style: Option<PrintStyle>,

    /// Configure how many commands are displayed at a time
    #[clap(long = "display-limit", short = 'd')]
    pub display_limit: Option<i32>,
}
