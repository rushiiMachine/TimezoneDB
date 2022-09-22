#[macro_use]
extern crate rocket;

use rocket::{Build, Config, Rocket};
use rocket::fairing::AdHoc;

mod constants;
mod routes;
mod utils;
mod logic;
mod database;

#[launch]
fn rocket() -> Rocket<Build> {
    if let Err(e) = kankyo::init() {
        println!("{e}");
    };
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
        .attach(AdHoc::on_liftoff("Liftoff log", |_| Box::pin(async move {
            println!("Launched TimezoneDB!");
        })))
}
