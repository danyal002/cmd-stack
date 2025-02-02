use crate::command::config_command::ConfigArgs;
use clap::{Args, Parser, Subcommand};

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
    Update(SearchArgs),

    /// Delete a command in your stack
    Delete(SearchArgs),

    /// Search for a command in your stack
    Search(SearchArgs),

    /// Export stack to a JSON file
    Export(ImportExportArgs),

    /// Import stack from a JSON file
    Import(ImportExportArgs),

    #[clap(subcommand)]
    /// Modify the config values
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

/// Arguments for searching and printing commands
#[derive(Debug, Args, Clone)]
pub struct SearchArgs {
    /// The text used to filter by command when searching
    pub command: Option<String>,

    /// The text used to filter by tag when searching
    #[clap(long = "tag", short = 't')]
    pub tag: Option<String>,

    /// Display commands in order of most recent use
    #[clap(long = "recent", short = 'r', action)]
    pub order_by_recently_used: bool,

    /// Only display favourite commands
    #[clap(long = "favourite", short = 'f', action)]
    pub favourite: bool,
}

/// Arguments for importing/exporting commands
#[derive(Debug, Args)]
pub struct ImportExportArgs {
    /// The relative path of the file
    pub file: String,
}
