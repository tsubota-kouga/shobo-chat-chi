extern crate chrono;
extern crate diesel;
extern crate diesel_migrations;

pub mod general;
pub use general::{run_db_migrations, MessagesDb};

pub mod message;
pub use message::Message;
