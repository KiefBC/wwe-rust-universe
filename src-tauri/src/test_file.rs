#[cfg(test)]
mod tests {
    use crate::models::{NewUser};
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;
    use crate::db::{create_user, check_user_exists, establish_connection, verify_credentials};
    use serial_test::serial;

    // Helper function to reset and establish the connection
    fn setup_test_user<'a>() -> (SqliteConnection, NewUser<'a>) {
        let mut conn = establish_connection();
        let test_user = NewUser {
            username: "Testing",
            password: "Testing",
        };

        reset_test_user(&mut conn, &test_user);
        (conn, test_user)
    }

    // Resets the test user by deleting it if it exists
    fn reset_test_user(conn: &mut SqliteConnection, test_user: &NewUser) {
        let result = diesel::delete(users.filter(username.eq(test_user.username)))
            .filter(password.eq(test_user.password))
            .execute(conn)
            .expect("Error deleting test user");

        println!("Deleted {} user", result);
    }

    #[test]
    #[serial]
    // Test to create a new user
    fn test_create_user() {
        println!("Creating new user.... first deleting if it exists");

        let (mut conn, new_user) = setup_test_user();
        println!("Creating new user");
        let user = create_user(&mut conn, new_user);

        assert_eq!(user.username, "Testing");
        assert_ne!(user.username, "Testing1");
    }

    #[test]
    #[serial]
    // Test to check if user exists
    fn test_check_user_exists() {
        println!("Checking if user exists");

        let (mut conn, new_user) = setup_test_user();
        create_user(&mut conn, new_user);

        let username_check: &str = "Testing";
        let username_check1: &str = "Testing1";

        let result = check_user_exists(&mut conn, username_check);
        let result1 = check_user_exists(&mut conn, username_check1);

        assert!(result);
        assert!(!result1);
    }

    #[test]
    #[serial]
    // Test to verify credentials of a user
    fn test_verify_credentials() {
        println!("Verifying credentials");

        let (mut conn, new_user) = setup_test_user();
        create_user(&mut conn, new_user);

        let username_check: &str = "Testing";
        let password_check: &str = "Testing";
        let password_check1: &str = "Testing1";

        assert!(verify_credentials(username_check.to_string(), password_check.to_string()));
        assert!(!verify_credentials(username_check.to_string(), password_check1.to_string()));
    }
}