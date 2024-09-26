use crate::models::{NewUser, User};
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

    // Establish the SQLite connection
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
            // Log the success message when the user is created
            info!("User '{}' created successfully", user.username);  // Assuming `user` has a `username` field
            Some(user)
        },
        Err(e) => {
            // Log the error if the insertion fails
            error!("Error saving new user: {}", e);
            None
        }
    }
}
