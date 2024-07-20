pub mod add_command;
pub mod delete_command;
pub mod list_commands;
pub mod search_command;
pub mod search_utils;
pub mod update_command;

use colored::Colorize;
use data::models::InternalCommand;

pub fn print_internal_command(internal_command: &InternalCommand) {
    println!("{} {}", "Command:".bold(), internal_command.command);
    println!("{} {}", "Alias:".bold(), internal_command.alias);
    if let Some(tag) = &internal_command.tag {
        println!("{} {}", "Tag:".bold(), tag);
    }
    if let Some(note) = &internal_command.note {
        println!("{} {}", "Note:".bold(), note);
    }
    println!(
        "{} {}\n",
        "Favourite:".bold(),
        if internal_command.favourite {
            "YES"
        } else {
            "NO"
        }
    )
}
