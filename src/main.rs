#[macro_use]
extern crate rocket;

use rocket::{Build, Config, Rocket};

mod constants;
mod routes;
mod utils;
mod logic;
mod database;

#[launch]
fn rocket() -> Rocket<Build> {
    let figment = Config::figment()
        .merge(("port", *constants::PORT))
        .merge(("databases.main.url", &*constants::POSTGRES_URL))
        .merge(("ident", false))
        .merge(("address", "0.0.0.0"));

    #[cfg(not(debug_assertions))]
        let figment = figment.merge(("log_level", "off"));

    rocket::custom(figment)
        .attach(database::setup())
        .attach(routes::setup())
}
