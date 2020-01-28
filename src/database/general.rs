use log::error;
use rocket::Rocket;
use rocket_contrib::database;
embed_migrations!();

#[database("messages_db")]
pub struct MessagesDb(diesel::SqliteConnection);

pub fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = MessagesDb::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}
