// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate diesel;

use tauri::command;
use log::error;
// use diesel::prelude::*;
use crate::db::establish_connection;
use crate::user::{create_user, find_username};

mod db;
mod user;
mod schema;// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

/*
Verifies the credentials of a user via the database
 */
#[command]
fn verify_credentials(username: String, password: String) -> bool {
    let mut connection = establish_connection();
    let user = user::get_user(&mut connection, &username);
    match user {
        Some(u) => {
            if u.password == password {
                true
            } else {
                false
            }
        }
        None => {
            error!("User not found");
            false
        }
    }
}

/*
Inserts a new user into the database
 */
#[command]
fn register_user(username: String, password: String) -> Result<bool, String> {
    let mut connection = establish_connection();

    println!("Finding username in database via register_user() main.rs");
    match find_username(&mut connection, &username) {

        Ok(Some(_)) => {
            // User already exists
            return Err(format!("User {} already exists", username));
        },

        Ok(None) => {
            println!("Creating user in database via register_user() x2");
            // Username is available
            match create_user(&mut connection, &username, &password) {
                Ok(_) => Ok(true),
                Err(err) => Err(format!("Error creating user: {}", err)),
            }
        },
        Err(err) => Err(format!("Error finding user: {}", err)),
    }
}

fn main() {
    // connect to the database
    let connection = establish_connection();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![verify_credentials, register_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
