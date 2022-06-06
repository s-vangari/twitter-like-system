mod routes;
mod db;
mod service;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate serde_json;

use rocket::{Build, Rocket};


#[launch]
fn rocket() -> Rocket<Build> {
    routes::rocket()
}
