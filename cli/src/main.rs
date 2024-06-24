//! # CLI
//!
//! This crate handles user interaction in the terminal

mod add_command;
mod args;

use crate::add_command::handle_add;
use args::{CmdStackArgs, Command};
use clap::Parser;

fn main() {
    let args = CmdStackArgs::parse();

    match args.command {
        Command::Add(add_args) => handle_add(add_args),
        Command::Update(update_args) => {
            println!("Update command: {:?}", update_args);
        }
        Command::Delete(delete_args) => {
            println!("Delete command: {:?}", delete_args);
        }
        Command::Search(search_args) => {
            println!("Search command: {:?}", search_args);
        }
        Command::List(list_args) => {
            println!("List command: {:?}", list_args);
        }
        Command::Param(param_args) => {
            println!("Param command: {:?}", param_args);
        }
    }
}


