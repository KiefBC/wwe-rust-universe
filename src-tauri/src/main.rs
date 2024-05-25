// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn verify_credentials(username: String, password: String) -> bool {
    if username == "admin" && password == "admin" {
        true
    } else {
        false
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![verify_credentials])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
