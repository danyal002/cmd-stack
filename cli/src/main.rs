//! # CLI
//!
//! This crate handles user interaction in the terminal

mod args;
mod command;
mod import_export;
pub mod outputs;
mod param;

use args::{CmdStackArgs, Command};
use clap::Parser;

fn main() {
    env_logger::init();

    let args = CmdStackArgs::parse();

    inquire::set_global_render_config(inquire::ui::RenderConfig {
        prompt: inquire::ui::StyleSheet::default().with_fg(inquire::ui::Color::LightBlue),
        ..Default::default()
    });

    match args.command {
        Command::Add(add_args) => command::add_command::handle_add_command(add_args),
        Command::Update(update_args) => command::update_command::handle_update_command(update_args),
        Command::Delete(delete_args) => command::delete_command::handle_delete_command(delete_args),
        Command::Search(search_args) => {
            command::search_command::handle_search_commands(search_args)
        }
        Command::List(list_args) => command::list_commands::handle_list_commands(list_args),
        Command::Param(param_args) => param::handle_param_command(param_args),
        Command::Export(import_export_args) => {
            import_export::handle_export_command(import_export_args)
        }
        Command::Import(import_export_args) => {
            import_export::handle_import_command(import_export_args)
        }
    }
}
