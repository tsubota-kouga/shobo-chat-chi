#![feature(proc_macro_hygiene, decl_macro)]
extern crate diesel;
extern crate log;
extern crate rocket;
extern crate rocket_contrib;

use rocket::fairing::AdHoc;
use rocket_contrib::serve::StaticFiles;

use shobo_chat_chi::database::{run_db_migrations, MessagesDb};
use shobo_chat_chi::routes::message;

#[rocket::get("/")]
fn index() -> &'static str {
    "GET /"
}

fn main() {
    rocket::ignite()
        .mount("/", rocket::routes![index])
        .mount(
            "/messages",
            rocket::routes![
                message::get,
                message::all,
                message::new,
                message::delete,
                message::update,
            ],
        )
        .mount("/public", StaticFiles::from("./client/public"))
        .attach(MessagesDb::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .launch();
}
