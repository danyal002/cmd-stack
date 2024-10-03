// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use logic::command::handle_list_commands;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_commands() -> Result<Vec<String>, String> {
    let commands = match handle_list_commands(false, false) {
        Ok(c) => c,
        Err(e) => return Err(format!("Error listing commands: {:?}", e)),
    };
    let command_strings: Vec<String> = commands
        .iter()
        .map(|c| c.internal_command.alias.clone())
        .collect();

    Ok(command_strings)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, list_commands])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
