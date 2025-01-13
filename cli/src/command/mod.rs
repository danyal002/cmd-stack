pub mod add_command;
pub mod delete_command;
pub mod search_command;
pub mod search_utils;
pub mod update_command;

use data::models::InternalCommand;
use prettytable::{format, Attr, Cell, Row, Table};

pub fn print_internal_command(internal_command: &InternalCommand) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    table.add_row(Row::new(vec![
        Cell::new("Command:").with_style(Attr::Italic(true)),
        Cell::new(&internal_command.command),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Alias:").with_style(Attr::Italic(true)),
        Cell::new(&internal_command.alias),
    ]));
    if let Some(tag) = &internal_command.tag {
        table.add_row(Row::new(vec![
            Cell::new("Tag:").with_style(Attr::Italic(true)),
            Cell::new(tag),
        ]));
    }
    if let Some(note) = &internal_command.note {
        table.add_row(Row::new(vec![
            Cell::new("Note:").with_style(Attr::Italic(true)),
            Cell::new(note),
        ]));
    }
    let favourite_status = if internal_command.favourite { "*" } else { "" };
    table.add_row(Row::new(vec![
        Cell::new("Favourite:").with_style(Attr::Italic(true)),
        Cell::new(favourite_status),
    ]));

    table.printstd();
}
