// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate diesel;

use tauri::command;
use log::error;
// use diesel::prelude::*;
use crate::db::establish_connection;

mod db;
mod user;
mod schema;// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn verify_credentials(username: String, password: String) -> bool {
    if username == "admin" && password == "admin" {
        true
    } else {
        false
    }
}

#[command]
fn register_user(username: String, password: String) -> bool {
    let mut connection = establish_connection();
    user::create_user(&mut connection, &username, &password);
    true
}

fn main() {
    // connect to the database
    let connection = establish_connection();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![verify_credentials, register_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
