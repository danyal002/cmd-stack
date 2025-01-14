use data::models::{Command, InternalCommand};
use logic::Logic;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayCommand {
    pub id: i64,
    pub last_used: i64,
    pub alias: String,
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
            alias: c.internal_command.alias.clone(),
            command: c.internal_command.command.clone(),
            tag: c.internal_command.tag.clone(),
            note: c.internal_command.note.clone(),
            favourite: c.internal_command.favourite,
        }
    }
}

#[tauri::command]
fn list_commands() -> Result<Vec<DisplayCommand>, String> {
    let logic = Logic::try_default().map_err(|e| format!("Failed to initialize Logic: {:?}", e))?;

    let commands = logic
        .list_commands(false, false)
        .map_err(|e| format!("Error listing commands: {:?}", e))?;

    let commands: Vec<DisplayCommand> = commands.iter().map(DisplayCommand::from).collect();
    Ok(commands)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCommand {
    pub alias: String,
    pub command: String,
    pub tag: Option<String>,
    pub note: Option<String>,
    pub favourite: bool,
}

impl From<&AddCommand> for InternalCommand {
    fn from(c: &AddCommand) -> Self {
        InternalCommand {
            alias: c.alias.clone(),
            command: c.command.clone(),
            tag: c.tag.clone(),
            note: c.note.clone(),
            favourite: c.favourite,
        }
    }
}

#[tauri::command]
fn add_command(command: AddCommand) -> Result<(), String> {
    let logic = Logic::try_default().map_err(|e| format!("Failed to initialize Logic: {:?}", e))?;

    let internal_command = InternalCommand::from(&command);

    logic
        .add_command(internal_command)
        .map_err(|e| format!("Error adding command: {:?}", e))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCommand {
    pub id: i64,
}

#[tauri::command]
fn delete_command(command: DeleteCommand) -> Result<(), String> {
    let logic = Logic::try_default().map_err(|e| format!("Failed to initialize Logic: {:?}", e))?;

    logic
        .delete_command(command.id)
        .map_err(|e| format!("Error deleting command: {:?}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            list_commands,
            add_command,
            delete_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
