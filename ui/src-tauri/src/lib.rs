use data::models::{Command, InternalCommand};
use logic::{
    command::{AddCommandError, DeleteCommandError, ListCommandError, UpdateCommandError},
    param::{ParameterError, SerializableParameter},
    Logic, LogicInitError,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UIError {
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
}

// we must manually implement serde::Serialize (https://github.com/tauri-apps/tauri/discussions/8805)
impl serde::Serialize for UIError {
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
fn list_commands() -> Result<Vec<DisplayCommand>, UIError> {
    let logic = Logic::try_default()?;
    let commands = logic.list_commands(false, false)?;
    let commands: Vec<DisplayCommand> = commands.iter().map(DisplayCommand::from).collect();
    Ok(commands)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCommand {
    pub command: String,
    pub tag: Option<String>,
    pub note: Option<String>,
    pub favourite: bool,
}

impl From<&ExternalCommand> for InternalCommand {
    fn from(c: &ExternalCommand) -> Self {
        InternalCommand {
            command: c.command.clone(),
            tag: c.tag.clone(),
            note: c.note.clone(),
            favourite: c.favourite,
        }
    }
}

#[tauri::command]
fn add_command(command: ExternalCommand) -> Result<(), UIError> {
    let logic = Logic::try_default()?;
    let internal_command = InternalCommand::from(&command);
    logic.add_command(internal_command)?;
    Ok(())
}

#[tauri::command]
fn update_command(command_id: i64, command: ExternalCommand) -> Result<(), UIError> {
    let logic = Logic::try_default()?;
    let internal_command = InternalCommand::from(&command);
    logic.update_command(command_id, internal_command)?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCommand {
    pub id: i64,
}

#[tauri::command]
fn delete_command(command: DeleteCommand) -> Result<(), UIError> {
    let logic = Logic::try_default()?;
    logic.delete_command(command.id)?;
    Ok(())
}

#[tauri::command]
fn parse_parameters(command: String) -> Result<(Vec<String>, Vec<SerializableParameter>), UIError> {
    let logic = Logic::try_default()?;
    let ret = logic.parse_parameters(command)?;
    Ok(ret)
}

#[tauri::command]
fn replace_parameters(command: String) -> Result<(String, Vec<String>), UIError> {
    let logic = Logic::try_default()?;
    let ret = logic.generate_parameters(command)?;
    Ok(ret)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            list_commands,
            add_command,
            delete_command,
            replace_parameters,
            parse_parameters,
            update_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
