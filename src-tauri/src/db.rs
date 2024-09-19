use diesel::prelude::*;
use dotenvy::dotenv;
use crate::models::{NewUser, User};
use std::env;

pub fn establish_connection() -> SqliteConnection {
    println!("CHECKING DB CONNECTION");
    dotenv().ok().expect("Error loading .env file");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);  // Print the URL to verify
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tauri::command]
pub fn create_user(conn: &mut SqliteConnection, new_user: NewUser) -> User {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn check_user_exists(conn: &mut SqliteConnection, username_check: &str) -> bool {
    use crate::schema::users::dsl::*;

    let result = users.filter(username.eq(username_check))
        .select(User::as_select())
        .load(conn)
        .expect("Error loading user");

    for field in &result {
        if field.username == username_check {
            println!("User exists");
            return true
        }
    }

    false
}

#[tauri::command]
pub fn verify_credentials(susername: String, spassword: String) -> bool {
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;  // Import diesel prelude for query execution
    use diesel::sqlite::Sqlite;
    use diesel::debug_query;

    println!("Verifying credentials...");
    println!("Username: {}", susername);
    println!("Password: {}", spassword);

    // Print the generated SQL for debugging purposes
    let query = users.filter(username.eq(&susername)).select(User::as_select());
    println!("Generated SQL: {:?}", debug_query::<Sqlite, _>(&query));

    let conn = &mut establish_connection();
    let result = users
        .filter(username.eq(&susername))
        .first::<User>(conn)  // Try to load the first matching user
        .optional();  // This will return Option<User>

    match result {
        Ok(Some(user)) => {
            if user.password == spassword {
                println!("Credentials verified");
                true
            } else {
                println!("Invalid password");
                false
            }
        }
        Ok(None) => {
            println!("User not found");
            false  // Return false if no user was found
        }
        Err(err) => {
            println!("Error loading user: {:?}", err);
            false
        }
    }
}

#[tauri::command]
pub fn register_user(susername: String, spassword: String) -> bool {
    let conn = &mut establish_connection();
    let new_user = NewUser {
        username: &susername,
        password: &spassword,
    };

    if check_user_exists(conn, &susername) {
        println!("User already exists");
        return false;
    }

    create_user(conn, new_user);
    true
}
