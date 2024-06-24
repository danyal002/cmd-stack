//! # CLI
//!
//! This crate handles user interaction in the terminal

mod add_command;
mod args;
mod list_commands;
mod search_command;
mod search_utils;

use crate::add_command::handle_add_command;
use crate::list_commands::handle_list_commands;
use crate::search_command::handle_search_commands;
use args::{CmdStackArgs, Command};
use clap::Parser;

fn main() {
    let args = CmdStackArgs::parse();

    match args.command {
        Command::Add(add_args) => handle_add_command(add_args),
        Command::Update(update_args) => {
            println!("Update command: {:?}", update_args);
        }
        Command::Delete(delete_args) => {
            println!("Delete command: {:?}", delete_args);
        }
        Command::Search(search_args) => handle_search_commands(search_args),
        Command::List(list_args) => handle_list_commands(list_args),
        Command::Param(param_args) => {
            println!("Param command: {:?}", param_args);
        }
    }
}
