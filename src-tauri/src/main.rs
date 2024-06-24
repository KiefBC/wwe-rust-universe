// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;

use tauri::command;
use log::error;
use crate::db::establish_connection;
use crate::models::{create_user, find_username};

pub mod db;
pub mod models;
pub mod schema;

/**
 * Verifies the credentials of a user via the database.
 *
 * # Arguments
 *
 * * `username` - A string slice that holds the username of the user.
 * * `password` - A string slice that holds the password of the user.
 *
 * # Returns
 *
 * * `bool` - Returns true if the credentials are valid, otherwise false.
 */
#[command]
fn verify_credentials(username: String, password: String) -> bool {
    let mut connection = establish_connection();
    let user = models::get_user(&mut connection, &username);
    match user {
        Some(user) => {
            if user.password == password {
                println!("User verified");
                true
            } else {
                println!("User not verified");
                false
            }
        }
        None => {
            error!("User not found");
            false
        }
    }
}

/**
 * Inserts a new user into the database.
 *
 * # Arguments
 *
 * * `username` - A string slice that holds the username of the new user.
 * * `password` - A string slice that holds the password of the new user.
 *
 * # Returns
 *
 * * `Result<bool, String>` - Returns Ok(true) if the user is successfully created, otherwise returns an Err with a string describing the error.
 */
#[command]
fn register_user(username: String, password: String) -> Result<bool, String> {
    let mut connection = establish_connection();

    match find_username(&mut connection, &username) {
        Ok(Some(_)) => {
            // User already exists
            return Err(format!("User {} already exists", username));
        }
        Ok(None) => {
            // Username is available
            match create_user(&mut connection, &username, &password) {
                Ok(_) => Ok(true),
                Err(err) => Err(format!("Error creating user: {}", err)),
            }
        }
        Err(err) => Err(format!("Error finding user: {}", err)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![verify_credentials, register_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
