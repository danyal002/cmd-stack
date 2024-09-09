use data::models::Parameter;
use inquire::InquireError;
use inquire::Select;
use prettytable::{format, Cell, Row, Table};
use termion::terminal_size;
use thiserror::Error;

use crate::utils::truncate_string;

#[derive(Error, Debug)]
pub enum ParamUtilError {
    #[error("No parameters")]
    NoParams,

    #[error("failed to get selected item")]
    GetSelectedItemFromUserError(#[from] InquireError),
}

pub fn list_parameters(params: Vec<Parameter>, print_limit: u32) -> Result<(), ParamUtilError> {
    if params.is_empty() {
        return Err(ParamUtilError::NoParams);
    }

    let formatted_params = format_params_for_printing(&params);

    println!(); // Spacing
    match Select::new("Parameters (Symbol | Regex | Note):", formatted_params)
        .with_formatter(&|_| "".to_string())
        .with_page_size(print_limit as usize)
        .raw_prompt()
    {
        Ok(_) => {}
        Err(e) => return Err(ParamUtilError::GetSelectedItemFromUserError(e)),
    };

    Ok(())
}

pub fn select_parameters(
    params: &Vec<Parameter>,
    print_limit: u32,
) -> Result<Parameter, ParamUtilError> {
    if params.is_empty() {
        return Err(ParamUtilError::NoParams);
    }

    let formatted_params = format_params_for_printing(params);

    println!(); // Spacing
    let selected_param = match Select::new(
        "Select a parameter (Symbol | Regex | Note):",
        formatted_params,
    )
    .with_formatter(&|i| params[i.index].internal_parameter.symbol.to_string())
    .with_page_size(print_limit as usize)
    .raw_prompt()
    {
        Ok(p) => p,
        Err(e) => return Err(ParamUtilError::GetSelectedItemFromUserError(e)),
    };

    Ok(params[selected_param.index].clone())
}

fn format_params_for_printing(params: &Vec<Parameter>) -> Vec<String> {
    let (width, _) = terminal_size().unwrap_or((150, 0)); // Default to 150 if terminal size cannot be determined

    println!("{}", width);

    // Define maximum widths for each column
    let symbol_width = std::cmp::max(width * 10 / 100, 15) as i32; // Alias gets 10% of width or 15, whichever is more
    let regex_width = std::cmp::max(width * 60 / 100, 30) as i32; // Regex gets 60% of the width or 30, whichever is more
    let note_width = std::cmp::max(width as i32 - symbol_width - regex_width - 12, 0); // Note gets remaining width

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    for param in params {
        let truncated_symbol =
            truncate_string(&param.internal_parameter.symbol, symbol_width as usize);
        let truncated_regex =
            truncate_string(&param.internal_parameter.regex, regex_width as usize);
        let truncated_note = truncate_string(
            param.internal_parameter.note.as_deref().unwrap_or(""),
            note_width as usize,
        );

        table.add_row(Row::new(vec![
            Cell::new(&truncated_symbol),
            Cell::new(&truncated_regex),
            Cell::new(&truncated_note),
        ]));
    }

    let table_str = table.to_string();
    return table_str.lines().map(|s| s.to_string()).collect();
}
