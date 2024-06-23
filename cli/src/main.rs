//! # CLI
//! 
//! This crate handles user interaction in the terminal 

mod args;
mod add;

use args::{CmdStackArgs, Command};
use clap::Parser;
use crate::add::handle_add;

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
