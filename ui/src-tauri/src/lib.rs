use logic::new_logic;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: i64,
    pub last_used: i64,
    pub alias: String,
    pub command: String,
    pub tag: Option<String>,
    pub note: Option<String>,
    pub favourite: bool,
}

#[tauri::command]
fn list_commands() -> Result<Vec<Command>, String> {
    let logic = match new_logic() {
        Ok(l) => l,
        Err(e) => return Err(format!("Failed to initialize Logic: {:?}", e)),
    };

    let commands = match logic.handle_list_commands(false, false) {
        Ok(c) => c,
        Err(e) => return Err(format!("Error listing commands: {:?}", e)),
    };
    let commands: Vec<Command> = commands
        .iter()
        .map(|c| Command { id: c.id, last_used: c.last_used, alias: c.internal_command.alias.clone(), command: c.internal_command.command.clone(), tag: c.internal_command.tag.clone(), note: c.internal_command.note.clone(), favourite: c.internal_command.favourite})
        .collect();

    Ok(commands)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![list_commands])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
