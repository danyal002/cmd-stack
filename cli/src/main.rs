//! # CLI
//!
//! This crate handles user interaction in the terminal

mod add_command;
mod args;
mod delete_command;
mod list_commands;
mod param;
mod search_command;
mod search_utils;
mod update_command;
mod import_export;

use args::{CmdStackArgs, Command};
use clap::Parser;

fn main() {
    let args = CmdStackArgs::parse();

    inquire::set_global_render_config(inquire::ui::RenderConfig {
        prompt: inquire::ui::StyleSheet::default().with_fg(inquire::ui::Color::LightBlue),
        ..Default::default()
    });

    match args.command {
        Command::Add(add_args) => add_command::handle_add_command(add_args),
        Command::Update(update_args) => update_command::handle_update_command(update_args),
        Command::Delete(delete_args) => delete_command::handle_delete_command(delete_args),
        Command::Search(search_args) => search_command::handle_search_commands(search_args),
        Command::List(list_args) => list_commands::handle_list_commands(list_args),
        Command::Param(param_args) => param::handle_param_command(param_args),
        Command::Export(export_args) => import_export::handle_export_command(export_args),
    }
}
