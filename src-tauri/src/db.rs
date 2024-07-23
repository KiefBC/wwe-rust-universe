use diesel::prelude::*;
use dotenv::dotenv;
use dotenv_codegen::dotenv;
use crate::models::{NewUser, User};

pub fn establish_connection() -> PgConnection {
    dotenv().ok().expect("Error loading .env file");

    let database_url = dotenv!("DATABASE_URL");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tauri::command]
pub fn create_user(conn: &mut PgConnection, new_user: NewUser) -> User {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn check_user_exists(conn: &mut PgConnection, username_check: &str) -> bool {
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
    println!("Verifying credentials...");

    let conn = &mut establish_connection();
    let results = users.filter(username.eq(&susername))
        .filter(password.eq(&spassword))
        .select(User::as_select())
        .load(conn)
        .expect("Error loading user");

    for field in &results {
        if field.username == susername && field.password == spassword {
            println!("ID: {}", field.id);
            println!("Username: {}", field.username);
            println!("Password: {}", field.password);
            println!("Created at: {}", field.created_at);
            return true
        }
    }
    false
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
