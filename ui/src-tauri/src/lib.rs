use std::sync::RwLock;

use data::models::{Command, InternalCommand};
use logic::{
    command::{
        AddCommandError, DeleteCommandError, ListCommandError, SearchCommandArgs,
        SearchCommandError, UpdateCommandError,
    },
    config::{Config, ConfigWriteError},
    parameters::{parser::SerializableParameter, ParameterError},
    Logic, LogicInitError,
};
use serde::{Deserialize, Serialize};
use tauri::State;
use thiserror::Error;

pub struct Ui {
    logic: RwLock<Logic>,
}

#[derive(Error, Debug)]
pub enum UiError {
    #[error("Failed to initialize logic")]
    LogicInit(#[from] LogicInitError),
    #[error("Failed to parse parameters")]
    Parse(#[from] ParameterError),
    #[error("Failed to delete command")]
    DeleteCommand(#[from] DeleteCommandError),
    #[error("Failed to add command")]
    AddCommand(#[from] AddCommandError),
    #[error("Failed to list commands")]
    ListCommand(#[from] ListCommandError),
    #[error("Failed to update command")]
    UpdateCommand(#[from] UpdateCommandError),
    #[error("Failed to search command")]
    SearchCommand(#[from] SearchCommandError),
    #[error("Failed to write config")]
    ConfigWriteError(#[from] ConfigWriteError),
    #[error("Failed to obtain lock to complete the required action")]
    Race,
}

// we must manually implement serde::Serialize (https://github.com/tauri-apps/tauri/discussions/8805)
impl serde::Serialize for UiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayCommand {
    pub id: i64,
    pub last_used: i64,
    pub command: String,
    pub tag: Option<String>,
    pub note: Option<String>,
    pub favourite: bool,
}

impl From<&Command> for DisplayCommand {
    fn from(c: &Command) -> Self {
        DisplayCommand {
            id: c.id,
            last_used: c.last_used,
            command: c.internal_command.command.clone(),
            tag: c.internal_command.tag.clone(),
            note: c.internal_command.note.clone(),
            favourite: c.internal_command.favourite,
        }
    }
}

#[tauri::command]
fn list_commands(state: State<Ui>) -> Result<Vec<DisplayCommand>, UiError> {
    if let Ok(logic) = state.logic.write() {
        return Ok(logic
            .list_commands(false, false)?
            .iter()
            .map(DisplayCommand::from)
            .collect());
    }
    Err(UiError::Race)
}

#[tauri::command]
fn add_command(command: InternalCommand, state: State<Ui>) -> Result<(), UiError> {
    if let Ok(logic) = state.logic.write() {
        return Ok(logic.add_command(command)?);
    }
    Err(UiError::Race)
}

#[tauri::command]
fn update_command(
    command_id: i64,
    command: InternalCommand,
    state: State<Ui>,
) -> Result<(), UiError> {
    if let Ok(logic) = state.logic.write() {
        return Ok(logic.update_command(command_id, command)?);
    }
    Err(UiError::Race)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCommand {
    pub id: i64,
}

#[tauri::command]
fn delete_command(command: DeleteCommand, state: State<Ui>) -> Result<(), UiError> {
    if let Ok(logic) = state.logic.write() {
        return Ok(logic.delete_command(command.id)?);
    }
    Err(UiError::Race)
}

#[tauri::command]
fn parse_parameters(
    command: String,
    state: State<Ui>,
) -> Result<(Vec<String>, Vec<SerializableParameter>), UiError> {
    if let Ok(logic) = state.logic.write() {
        return Ok(logic.parse_parameters(command)?);
    }
    Err(UiError::Race)
}

#[tauri::command]
fn replace_parameters(command: String, state: State<Ui>) -> Result<(String, Vec<String>), UiError> {
    if let Ok(logic) = state.logic.write() {
        return Ok(logic.generate_parameters(command)?);
    }
    Err(UiError::Race)
}

#[tauri::command]
fn search_commands(search: String, state: State<Ui>) -> Result<Vec<DisplayCommand>, UiError> {
    if let Ok(logic) = state.logic.write() {
        let commands = logic
            .search_command(SearchCommandArgs {
                command: if search.is_empty() {
                    None
                } else {
                    Some(search)
                },
                tag: None,
                order_by_recently_used: false,
                favourites_only: false,
            })?
            .iter()
            .map(DisplayCommand::from)
            .collect();

        return Ok(commands);
    }
    Err(UiError::Race)
}

#[tauri::command]
fn read_config(state: State<Ui>) -> Result<Config, UiError> {
    if let Ok(logic) = state.logic.write() {
        return Ok(logic.config);
    }
    Err(UiError::Race)
}

#[tauri::command]
fn write_config(config: Config, state: State<Ui>) -> Result<(), UiError> {
    if let Ok(mut logic) = state.logic.write() {
        logic.config = config;
        return Ok(logic.config.write()?);
    }
    Err(UiError::Race)
}

#[tauri::command]
fn open_terminal(command: String) {
    std::process::Command::new("osascript")
        .arg("-e")
        .arg(format!(
            "tell application \"Terminal\" to activate"
        ))
        .arg("-e")
        .arg(format!(
            "tell application \"System Events\" to keystroke \"{}\"",
            command
        ))
        .spawn()
        .unwrap();
}

#[tauri::command]
fn update_command_last_used(command_id: i64, state: State<Ui>) -> Result<(), UiError> {
    if let Ok(logic) = state.logic.write() {
        return Ok(logic.update_command_last_used_prop(command_id)?);
    }
    Err(UiError::Race)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let logic = Logic::try_default()
        .map_err(|e| panic!("Failed to initialize Logic: {}", e))
        .unwrap();

    tauri::Builder::default()
        .manage(Ui {
            logic: logic.into(),
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            list_commands,
            add_command,
            delete_command,
            replace_parameters,
            parse_parameters,
            update_command,
            search_commands,
            read_config,
            write_config,
            update_command_last_used,
            open_terminal
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
