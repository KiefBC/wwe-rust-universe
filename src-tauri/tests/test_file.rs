#[cfg(test)]
mod tests {
    // extern crate wwe_rust_universe;

    use wwe_rust_universe::auth::{check_user_exists, verify_credentials};
    use wwe_rust_universe::db::{create_user, create_wrestler, establish_connection, create_belt};
    use wwe_rust_universe::models::{NewTitle, NewUser, NewWrestler, User, Wrestler, Title};
    use wwe_rust_universe::schema::users::dsl::{users, username, password};
    use wwe_rust_universe::schema::wrestlers::dsl::{wrestlers, name as wrestler_name};
    use wwe_rust_universe::schema::belts::dsl::{belts, name as belt_name};
    use diesel::prelude::*;
    use log::info;
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

    fn setup_test_wrestler<'a>() -> (SqliteConnection, NewWrestler<'a>) {
        let mut conn = establish_connection();
        let test_wrestler = NewWrestler {
            name: "Testing",
            gender: "Male Test",
        };

        reset_test_wrestler(&mut conn, &test_wrestler);
        (conn, test_wrestler)
    }

    fn setup_test_belt<'a>() -> (SqliteConnection, NewTitle<'a>) {
        let mut conn = establish_connection();
        let test_belt = NewTitle {
            name: "Testing"
        };

        reset_test_belt(&mut conn, &test_belt);
        (conn, test_belt)
    }

    // Resets the test user by deleting it if it exists
    fn reset_test_user(conn: &mut SqliteConnection, test_user: &NewUser) {
        if let Ok(user) = users.filter(username.eq(test_user.username))
            .filter(password.eq(test_user.password))
            .first::<User>(conn)
        {
            println!("Deleting user: {:?}", user);
        }

        let result = diesel::delete(users.filter(username.eq(test_user.username)))
            .filter(password.eq(test_user.password))
            .execute(conn)
            .expect("Error deleting test user");

        info!("Deleted {} user", result);
    }

    fn reset_test_wrestler(conn: &mut SqliteConnection, test_wrestler: &NewWrestler) {
        if let Ok(wrestler) = wrestlers.filter(wrestler_name.eq(test_wrestler.name))
            .first::<Wrestler>(conn)
        {
            println!("Deleting wrestler: {:?}", wrestler);
        }

        let result = diesel::delete(wrestlers.filter(wrestler_name.eq(test_wrestler.name)))
            .execute(conn)
            .expect("Error deleting test wrestler");

        info!("Deleted {} wrestler", result);
    }

    fn reset_test_belt(conn: &mut SqliteConnection, test_belt: &NewTitle) {
        if let Ok(belt) = belts.filter(belt_name.eq(test_belt.name))
            .first::<Title>(conn)
        {
            println!("Deleting belt: {:?}", belt);
        }

        let result = diesel::delete(belts.filter(belt_name.eq(test_belt.name)))
            .execute(conn)
            .expect("Error deleting test belt");

        info!("Deleted {} belt", result);
    }

    #[test]
    #[serial]
    // Test to create a new user
    fn test_create_user() {
        let (mut conn, new_user) = setup_test_user();
        let user = create_user(&mut conn, new_user).expect("User not created");

        assert_eq!(user.username, "Testing");
        assert_eq!(user.password, "Testing");
    }

    #[test]
    #[serial]
    // Test to create a new Wrestler
    fn test_create_wrestler() {
        let (mut conn, new_wrestler) = setup_test_wrestler();
        info!("Creating new Wrestler");
        let wrestler = create_wrestler(&mut conn, new_wrestler).expect("Wrestler not created");

        assert_eq!(wrestler.name, "Testing");
        assert_ne!(wrestler.name, "Testing1");
    }

    #[test]
    #[serial]
    // Test to create a new Belt
    fn test_create_belt() {
        let (mut conn, new_belt) = setup_test_belt();
        info!("Creating new Belt");
        let belt = create_belt(&mut conn, new_belt).expect("Belt not created");

        assert_eq!(belt.name, "Testing");
        assert_ne!(belt.name, "Testing1");
    }

    #[test]
    #[serial]
    // Test to check if user exists
    fn test_check_user_exists() {
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
        let (mut conn, new_user) = setup_test_user();
        create_user(&mut conn, new_user);

        let username_check: &str = "Testing";
        let password_check: &str = "Testing";
        let password_check1: &str = "Testing1";

        assert!(verify_credentials(
            username_check.to_string(),
            password_check.to_string()
        ));
        assert!(!verify_credentials(
            username_check.to_string(),
            password_check1.to_string()
        ));
    }
}
