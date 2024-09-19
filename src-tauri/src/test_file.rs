#[cfg(test)]
mod tests {
    use crate::models::{NewUser};
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;
    use crate::db::{create_user, check_user_exists, establish_connection, verify_credentials};
    use serial_test::serial;

    fn reset_test_user(conn: &mut SqliteConnection)  {
        let test_user = NewUser { username: "Testing", password: "Testing" };
        let result = diesel::delete(users.filter(username.eq(test_user.username)))
            .filter(password.eq(test_user.password))
            .execute(conn)
            .expect("Error deleting test user");

        println!("Deleted {} user", result);
    }

    #[test]
    #[serial]
    fn test_create_user() {
        println!("Creating new user.... first deleting if it exists");
        let mut conn = establish_connection();
        reset_test_user(&mut conn);

        println!("Creating new user");
        let new_user = NewUser { username: "Testing", password: "Testing" };
        let user = create_user(&mut conn, new_user);

        assert_eq!(user.username, "Testing");
        assert_ne!(user.username, "Testing1");
    }

    #[test]
    #[serial]
    fn test_check_user_exists() {
        println!("Checking if user exists");
        let mut conn = establish_connection();
        reset_test_user(&mut conn);

        let new_user = NewUser { username: "Testing", password: "Testing" };
        create_user(&mut conn, new_user);

        let username_check: &str = "Testing";
        let username_check1: &str = "Testing1";

        let result = check_user_exists(&mut conn, username_check);
        let result1 = check_user_exists(&mut conn, username_check1);

        assert_eq!(result, true);
        assert_ne!(result1, true);
    }

    #[test]
    #[serial]
    fn test_verify_credentials() {
        println!("Verifying credentials");
        let mut conn = establish_connection();
        reset_test_user(&mut conn);

        let new_user = NewUser { username: "Testing", password: "Testing" };
        create_user(&mut conn, new_user);

        let username_check: &str = "Testing";
        let password_check: &str = "Testing";
        let password_check1: &str = "Testing1";

        assert_eq!(verify_credentials(username_check.to_string(), password_check.to_string()), true);
        assert_ne!(verify_credentials(username_check.to_string(), password_check1.to_string()), true);
    }
}