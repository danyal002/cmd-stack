use crate::{
    args::SearchArgs,
    outputs::{format_output, spacing},
    utils::{none_if_empty, truncate_string},
    Cli,
};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use data::models::Command;
use inquire::{InquireError, Select, Text};
use log::error;
use prettytable::{format, Cell, Row, Table};
use regex::Regex;
use termion::terminal_size;
use thiserror::Error;

pub struct SearchArgsUserInput {
    pub command: Option<String>,
    pub tag: Option<String>,
}
impl From<SearchArgs> for SearchArgsUserInput {
    fn from(args: SearchArgs) -> Self {
        SearchArgsUserInput {
            command: args.command,
            tag: args.tag,
        }
    }
}

/// Given the user input for `command` and `tag`, determine
/// if the user provided search arguments
///
/// Returns a boolean
pub fn check_search_args_exist(command: &Option<String>, tag: &Option<String>) -> bool {
    command.is_some() || tag.is_some()
}

#[derive(Error, Debug)]
pub enum PromptUserForCommandSelectionError {
    #[error("Cannot select on empty list of commands")]
    NoCommandsProvided,
    #[error("Failed to render")]
    Inquire(#[from] InquireError),
}

/// Handles the UI interaction to prompt the user for selection
///
/// `commands` must be non-empty
impl Cli {
    /// Generates a wizard to set the properties for command searching
    pub fn prompt_user_for_search_args(&self) -> Result<SearchArgsUserInput, InquireError> {
        spacing();

        let command = Text::new(&format_output(
            "<bold>Command</bold> <italics>(Leave blank for no filter)</italics><bold>:</bold>",
        ))
        .prompt()?;

        let tag = Text::new(&format_output(
            "<bold>Tag</bold> <italics>(Leave blank for no filter)</italics><bold>:</bold>",
        ))
        .prompt()?;

        Ok(SearchArgsUserInput {
            command: none_if_empty(command),
            tag: none_if_empty(tag),
        })
    }

    /// Prompt user to edit the generated command
    pub fn prompt_user_for_command_edit(
        &self,
        initial_value: &str,
    ) -> Result<String, InquireError> {
        Text::new(&format_output(
            "<bold>Edit Command</bold> <italics>(Press enter to continue)</italics><bold>:</bold> ",
        ))
        .with_initial_value(initial_value)
        .prompt()
    }

    /// Prompt user to select an action: Copy or Execute
    pub fn prompt_user_for_action(&self) -> Result<String, InquireError> {
        Ok(Select::new(
            &format_output("<bold>Select Action:</bold>"),
            vec!["Copy", "Execute"],
        )
        .prompt()?
        .to_owned())
    }

    pub fn prompt_user_for_command_selection(
        &self,
        commands: Vec<Command>,
    ) -> Result<Command, PromptUserForCommandSelectionError> {
        if commands.is_empty() {
            return Err(PromptUserForCommandSelectionError::NoCommandsProvided);
        }

        let (formatted_commands, columns) = self.format_commands_for_printing(&commands);

        spacing();
        let selected_command = match Select::new(
            &format_output(
                &("<bold>Select a command</bold> <italics>".to_owned()
                    + columns
                    + "</italics><bold>:</bold>"),
            ),
            formatted_commands,
        )
        // Only display the command once the user makes a selection
        .with_formatter(&|i| {
            format_output(
                &self.get_selected_command_string(&commands[i.index].internal_command.command),
            )
        })
        .with_page_size(self.logic.config.cli_display_limit as usize)
        .raw_prompt()
        {
            Ok(c) => c,
            Err(e) => {
                return Err(PromptUserForCommandSelectionError::Inquire(e));
            }
        };

        Ok(commands[selected_command.index].clone())
    }

    fn get_selected_command_string(&self, command: &str) -> String {
        let blank_param_regex = Regex::new(r"@\{\s*\}").unwrap();
        let mut blank_param_num = 1;

        let command_with_placeholders =
            blank_param_regex.replace_all(command, |_: &regex::Captures| {
                let replacement =
                    format!("<bold><italics>@{{{}}}</italics></bold>", blank_param_num);
                blank_param_num += 1;
                replacement
            });

        command_with_placeholders.to_string()
    }

    /// Formats the commands for printing based on the user's preferred style.
    /// Returns the columns to be printed
    fn format_commands_for_printing(&self, commands: &Vec<Command>) -> (Vec<String>, &str) {
        match self.logic.config.cli_print_style {
            logic::config::CliPrintStyle::All => (
                self.format_internal_commands(commands),
                "(Command | Tag | Note | Favourite [*])",
            ),
            logic::config::CliPrintStyle::CommandsOnly => (
                commands
                    .iter()
                    .map(|c| c.internal_command.command.clone())
                    .collect(),
                "(Command)",
            ),
        }
    }

    fn format_internal_commands(&self, commands: &Vec<Command>) -> Vec<String> {
        let (width, _) = terminal_size().unwrap_or((150, 0)); // Default to 150 if terminal size cannot be determined

        // Define maximum widths for each column
        let tag_width = std::cmp::max(width * 5 / 100, 8) as i32; // Tag gets 5% of the width or 8, whichever is more
        let favourite_width = 5;

        let remaining_width = std::cmp::max(width as i32 - tag_width - favourite_width - 12, 0);
        let command_width = remaining_width * 75 / 100; // Commands get 75% of remaining width
        let note_width = remaining_width - command_width;

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        for command in commands {
            let truncated_tag = truncate_string(
                command.internal_command.tag.as_deref().unwrap_or(""),
                tag_width as usize,
            );
            let truncated_command =
                truncate_string(&command.internal_command.command, command_width as usize);
            let truncated_note = truncate_string(
                command.internal_command.note.as_deref().unwrap_or(""),
                note_width as usize,
            );

            table.add_row(Row::new(vec![
                Cell::new(&truncated_command),
                Cell::new(&truncated_tag),
                Cell::new(&truncated_note),
                Cell::new(if command.internal_command.favourite {
                    "*"
                } else {
                    ""
                }),
            ]));
        }

        let table_str = table.to_string();
        table_str.lines().map(|s| s.to_string()).collect()
    }
}

#[derive(Error, Debug)]
pub enum CopyTextError {
    #[error("Failed to initialize the clipboard")]
    ClipboardInit,
    #[error("Failed to copy text to clipboard")]
    Copy,
}

pub fn copy_to_clipboard(text_to_copy: String) -> Result<(), CopyTextError> {
    let mut clipboard = ClipboardContext::new().map_err(|_| CopyTextError::ClipboardInit)?;

    clipboard
        .set_contents(text_to_copy.clone())
        .map_err(|_| CopyTextError::Copy)?;

    Ok(())
}
