use data::models::Parameter;
use inquire::InquireError;
use prettytable::{format, Cell, Row, Table};
use thiserror::Error;
use inquire::Select;

#[derive(Error, Debug)]
pub enum ParamUtilError {
    #[error("No parameters")]
    NoParams,

    #[error("failed to get selected item")]
    GetSelectedItemFromUserError(#[from] InquireError),
}

pub fn list_parameters(params: Vec<Parameter>) -> Result<(), ParamUtilError> {
    if params.len() == 0 {
        return Err(ParamUtilError::NoParams);
    }

    let formatted_params = format_params_for_printing(&params);

    println!("\nParameters (Symbol | Regex | Note):"); // Spacing
    for line in formatted_params {
        println!("{}", line);
    }

    return Ok(());
}

pub fn select_parameters(params: &Vec<Parameter>, print_limit: u32) -> Result<Parameter, ParamUtilError> {
    if params.len() == 0 {
        return Err(ParamUtilError::NoParams);
    }

    let formatted_params = format_params_for_printing(&params);

    println!(); // Spacing
    let selected_param = match Select::new(
        "Select a parameter (Symbol | Regex | Note):",
        formatted_params
    )
    .with_formatter(&|i| format!("{}", &params[i.index].internal_parameter.symbol))
    .with_page_size(print_limit as usize)
    .raw_prompt()
    {
        Ok(p) => p,
        Err(e) => {
            return Err(ParamUtilError::GetSelectedItemFromUserError(e))
        }
    };

    return Ok(params[selected_param.index].clone());
}

fn format_params_for_printing(params: &Vec<Parameter>) -> Vec<String> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    for param in params {
        table.add_row(Row::new(vec![
            Cell::new(&param.internal_parameter.symbol),
            Cell::new(&param.internal_parameter.regex),
            Cell::new(&param.internal_parameter.note.as_deref().unwrap_or("")),
        ]));
    }

    let table_str = table.to_string();
    return table_str.lines().map(|s| s.to_string()).collect();
}
