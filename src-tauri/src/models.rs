// tauri-src/models
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};
use tauri::command;

use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

/*
This function creates a new user in the database
 */
#[command]
pub fn create_user<'a>(conn: &mut SqliteConnection, username: &'a str, password: &'a str) -> Result<usize, Error> {
    use crate::schema::users;

    let new_user = NewUser {
        username,
        password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
}

/*
find_username function is a simple, straightforward function for finding a user in a database
by their username. It demonstrates how to construct and
execute a SQL query using Diesel's query builder, and how to handle the results.
 */
pub fn find_username(conn: &mut SqliteConnection, uname: &str) -> Result<Option<User>, Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(username.eq(uname))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

/*
This function retrieves a user from the database

NOTE: changed username to uname to avoid conflict with the username field in the User struct
 */
#[command]
pub fn get_user(conn: &mut SqliteConnection, uname: &str) -> Option<User> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(username.eq(uname))
        .first::<User>(conn)
        .optional()
        .expect("Error loading user");

    user
}