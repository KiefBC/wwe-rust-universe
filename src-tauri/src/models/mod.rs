use diesel::prelude::*;

mod title;
mod user;
mod wrestler;

// Re-export models
pub use title::{NewTitle, Title};
pub use user::{NewUser, User};
pub use wrestler::{NewWrestler, Wrestler};
