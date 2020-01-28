#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod database;

extern crate rocket;
extern crate rocket_contrib;
pub mod routes;
