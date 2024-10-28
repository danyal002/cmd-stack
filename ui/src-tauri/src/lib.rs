use logic::new_logic;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_commands() -> Result<Vec<String>, String> {
    let logic = match new_logic() {
        Ok(l) => l,
        Err(e) => return Err(format!("Failed to initialize Logic: {:?}", e)),
    };

    let commands = match logic.handle_list_commands(false, false) {
        Ok(c) => c,
        Err(e) => return Err(format!("Error listing commands: {:?}", e)),
    };
    let command_strings: Vec<String> = commands
        .iter()
        .map(|c| c.internal_command.alias.clone())
        .collect();

    Ok(command_strings)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, list_commands])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
