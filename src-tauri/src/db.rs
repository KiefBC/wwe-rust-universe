use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;
use dotenv::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    // print out in the terminal
    println!("Establishing connection to database");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // print out the database url
    println!("Connected to {}", &database_url);

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}