use clap:: {
    Args,
    Parser,
    Subcommand,
    ValueEnum,
};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct CmdStackArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
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
    Param(ParamCommands)
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// The command to add
    pub command: String,

    /// The command alias
    pub alias: Option<String>,

    /// The command description
    pub note: Option<String>,

    /// The command tag
    pub tag: Option<String>,

    /// Add the command to your favourites
    #[clap(long, short, action)]
    pub favourite: bool
}

#[derive(Debug, ValueEnum, Clone)]
pub enum PrintStyle {
    /// Display the alias, value, tag, and notes
    All,

    /// Only display the command
    Command,

    /// Only display the alias
    Alias
}

#[derive(Debug, Args)]
pub struct SearchAndPrintArgs {
    /// The command to add
    pub command: Option<String>,

    /// The command alias
    pub alias: Option<String>,

    /// The command tag
    pub tag: Option<String>,

    /// How commands should be displayed
    #[clap(value_enum, default_value_t=PrintStyle::All)]
    pub print_style: PrintStyle,

    /// The number of commands to list at a time
    #[clap(default_value="10")]
    pub display_limit: u32,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Choose how the commands should be displayed
    #[clap(value_enum, default_value_t=PrintStyle::All)]
    pub print_style: PrintStyle,

    /// The number of commands to list at a time
    #[clap(default_value="10")]
    pub display_limit: u32,

    /// Order the commands by most recent use
    #[clap(long, short, action)]
    pub recent: bool,

    /// Only display your favourite commands
    #[clap(long, short, action)]
    pub favourite: bool 
}

#[derive(Debug, Subcommand)]
pub enum ParamCommands {
    /// Add a parameter
    Add(SearchAndPrintArgs),

    /// Update a parameter
    Update(SearchAndPrintArgs),

    /// Delete a parameter
    Delete(SearchAndPrintArgs),

    /// List all parameters
    List(SearchAndPrintArgs)
}