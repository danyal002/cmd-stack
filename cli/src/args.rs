use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct CmdStackArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
/// Different commands that can be executed
pub enum Command {
    /// Add a command
    Add(AddArgs),

    /// Update a command
    Update(SearchAndPrintArgs),

    /// Delete a command
    Delete(SearchAndPrintArgs),

    /// Search for a command
    Search(SearchAndPrintArgs),

    /// List all commands
    List(ListArgs),

    /// Parameter generation management
    #[clap(subcommand)]
    Param(ParamCommands),

    /// Export commands to a file
    Export(ImportExportArgs),
}

#[derive(Debug, Args)]
/// Arguments for adding a command
pub struct AddArgs {
    /// The command to add
    pub command: String,

    /// The command alias
    #[clap(long = "alias", short = 'a')]
    pub alias: Option<String>,

    /// The command description
    #[clap(long = "note", short = 'n')]
    pub note: Option<String>,

    /// The command tag
    #[clap(long = "tag", short = 't')]
    pub tag: Option<String>,

    /// Add the command to your favourites
    #[clap(long = "favourite", short = 'f', action)]
    pub favourite: bool,
}

#[derive(Debug, ValueEnum, Clone)]
/// Different supported printing methods
pub enum PrintStyle {
    /// Display the alias, value, tag, and notes
    All,

    /// Only display the command
    Command,

    /// Only display the alias
    Alias,
}

#[derive(Debug, Args)]
/// Arguments for searching and printing commands
pub struct SearchAndPrintArgs {
    /// The command to add
    #[clap(long = "command", short = 'c')]
    pub command: Option<String>,

    /// The command alias
    #[clap(long = "alias", short = 'a')]
    pub alias: Option<String>,

    /// The command tag
    #[clap(long = "tag", short = 't')]
    pub tag: Option<String>,

    /// How commands should be displayed
    #[clap(long="print-style", value_enum, default_value_t=PrintStyle::All)]
    pub print_style: PrintStyle,

    /// The number of commands to list at a time
    #[clap(long = "display-limit", default_value = "10")]
    pub display_limit: u32,
}

#[derive(Debug, Args)]
/// Arguments for listing commands
pub struct ListArgs {
    /// Choose how the commands should be displayed
    #[clap(long="print-style", value_enum, default_value_t=PrintStyle::All)]
    pub print_style: PrintStyle,

    /// The number of commands to list at a time
    #[clap(long = "display-limit", default_value = "10")]
    pub display_limit: u32,

    /// Order the commands by most recent use
    #[clap(long = "recent", short = 'r', action)]
    pub recent: bool,

    /// Only display your favourite commands
    #[clap(long = "favourite", short = 'f', action)]
    pub favourite: bool,
}

#[derive(Debug, Subcommand)]
/// Parameter management commands
pub enum ParamCommands {
    /// Add a parameter
    Add(SearchAndPrintArgs),

    /// Update a parameter
    Update(SearchAndPrintArgs),

    /// Delete a parameter
    Delete(SearchAndPrintArgs),

    /// List all parameters
    List(SearchAndPrintArgs),
}

#[derive(Debug, Args)]
/// Arguments for importing/exporting commands
pub struct ImportExportArgs {
    /// The file to export to
    pub file: String,
}
