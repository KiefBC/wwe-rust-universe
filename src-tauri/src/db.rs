use crate::models::{ NewUser, User, Wrestler, NewWrestler, Title, NewTitle };
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use log::{info, error};

pub fn establish_connection() -> SqliteConnection {
    println!("CHECKING DB CONNECTION");

    // Load environment variables from .env file
    dotenv().ok().expect("Error loading .env file");

    // Retrieve the DATABASE_URL environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url); // Print the URL to verify

    let mut conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // Enable foreign key constraints using a raw SQL query
    diesel::sql_query("PRAGMA foreign_keys = ON")
        .execute(&mut conn)
        .expect("Failed to enable foreign key constraints");

    conn
}

#[tauri::command]
pub fn create_user(conn: &mut SqliteConnection, new_user: NewUser) -> Option<User> {
    use crate::schema::users::dsl::*;

    match diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
    {
        Ok(user) => {
            info!("User '{}' created successfully", user.username);  // Assuming `user` has a `username` field
            Some(user)
        },
        Err(e) => {
            error!("Error saving new user: {}", e);
            None
        }
    }
}

#[tauri::command]
pub fn create_wrestler(conn: &mut SqliteConnection, new_wrestler: NewWrestler) -> Option<Wrestler> {
    use crate::schema::wrestlers::dsl::*;

    match diesel::insert_into(wrestlers)
        .values(&new_wrestler)
        .returning(Wrestler::as_returning())
        .get_result(conn)
    {
        Ok(wrestler) => {
            info!("Wrestler '{}' created successfully", wrestler.name);  // Assuming `wrestler` has a `name` field
            Some(wrestler)
        },
        Err(e) => {
            error!("Error saving new wrestler: {}", e);
            None
        }
    }
}

#[tauri::command]
pub fn create_belt(conn: &mut SqliteConnection, new_title: NewTitle) -> Option<Title> {
    use crate::schema::belts::dsl::*;

    match diesel::insert_into(belts)
        .values(&new_title)
        .returning(Title::as_returning())
        .get_result(conn)
    {
        Ok(title) => {
            info!("Title '{}' created successfully", title.name);  // Assuming `title` has a `name` field
            Some(title)
        },
        Err(e) => {
            error!("Error saving new title: {}", e);
            None
        }
    }
}