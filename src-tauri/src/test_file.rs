/**
This file contains tests for the functions in the main.rs file
Everything is sequential, so the tests will run in order
This is a requirement for the create_user function, as it relies on the check_user_exists function
*/

#[cfg(test)]
mod tests {
    use crate::models::{ NewUser };
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;
    use crate::db::{ create_user, check_user_exists, establish_connection };

    /**
    Testing the check_user_exists function
    Do this first, as the create_user function relies on it
    */
    #[test]
    fn test_check_user_exists() {
        println!("Checking if user exists");
        let conn = &mut establish_connection();

        // Check if the "Testing" user exists
        assert_eq!(check_user_exists(conn, "Testing"), true);
        assert_ne!(check_user_exists(conn, "Testing1"), true);
    }

    /*
    Testing the create_user function
    It will first delete the "Testing" user if it exists, then create it
     */
    #[test]
    fn test_create_user() {
        println!("Creating new user.... first deleting if it exists");
        if check_user_exists(&mut establish_connection(), "Testing") {
            let mut conn = establish_connection();
            diesel::delete(users.filter(username.eq("Testing")))
                .execute(&mut conn)
                .expect("Error deleting user");
        }

        println!("Creating new user");
        let mut conn = establish_connection();
        let new_user = NewUser { username: "Testing", password: "Testing" };
        let user = create_user(&mut conn, new_user);
        assert_eq!(user.username, "Testing");
        assert_ne!(user.username, "Testing1");
    }
}