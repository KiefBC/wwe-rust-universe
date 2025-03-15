# The beginning of something great

# WWE Universe Rust App

This is a simple Rust app that allows you to interact with the WWE Universe in ways you never thought possible.

# To Run

*This assumes you have Cargo, Rustup, & Diesel installed*

1. Clone the repo
2. `diesel setup` to setup the database
3. `diesel migration run` to run the migrations
4. Run `cargo tauri dev` to start the app

# Running Tests

You can run tests using the provided Makefile:

- `make test` - Run all tests for all packages
- `make test-frontend` - Run tests for the frontend package only
- `make test-backend` - Run tests for the backend package only

Alternatively, you can use the shell script:

- `./run_tests.sh` - Run all tests for all packages

Or use Cargo directly:

- `cargo test --workspace` - Run all tests for all packages
- `cargo test -p wwe-universe-manager` - Run tests for the backend package only
- `cargo test -p wwe-universe-manager-frontend` - Run tests for the frontend package only

## Project Structure and Testing

This project is set up as a Rust workspace with multiple packages:

- `wwe-universe-manager-frontend` - The frontend package (in the root directory)
- `wwe-universe-manager` - The backend package (in the `src-tauri` directory)

### Test Organization

- Backend tests are located in `src-tauri/tests/` and include:
  - `user_tests.rs` - Tests for user authentication and management
  - `wrestler_tests.rs` - Tests for wrestler creation and management
  - `title_tests.rs` - Tests for championship belt management
  - `common.rs` - Common test utilities and setup functions

### Understanding Test Output

When running tests, you'll see output for each package and test file. A successful test run will show:

```
test result: ok. X passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Where X is the number of tests that passed in that file.

### Adding New Tests

To add new tests:

1. For backend features, add test files in `src-tauri/tests/`
2. For frontend features, add test files in `src/tests/` (if applicable)
3. Use the common test utilities in `common.rs` for setup and teardown

### Continuous Integration

The test suite is designed to be run in CI environments. The `run_tests.sh` script will exit with a non-zero status code if any tests fail, making it suitable for CI pipelines.
