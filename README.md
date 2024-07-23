# The beginning of something great

# WWE Universe Rust App

This is a simple Rust app that allows you to interact with the WWE Universe in ways you never thought possible.

# Languages and Tools

- Rust
- Tauri
- Sveltekit
- Tailwind CSS
- SQLite (Diesel ORM)
- Typescript

# To Run

*This assumes you have Cargo, Rustup installed*

1. Clone the repo
2. `cargo install diesel_cli --no-default-features --features postgres` to install the diesel CLI
3. `diesel setup` to setup the database
4. `diesel migration run` to run the migrations 
5. Run `pnpm install` to install the dependencies 
6. Run `pnpm tauri dev` to start the app 
7. Enjoy!