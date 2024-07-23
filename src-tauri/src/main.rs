// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::{verify_credentials, register_user};

mod models;
mod schema;
mod test_file;
mod db;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![verify_credentials, register_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
