#[macro_use]
extern crate rocket;

use std::net::{IpAddr, Ipv4Addr};

use rocket::Config;
use rocket::config::LogLevel;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;

mod constants;
mod routes;
mod utils;

#[get("/")]
async fn index() -> Value {
    json!({"status": "online"})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(routes::user::routes())
        .attach(routes::auth::routes())
        .configure(Config {
            address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: *constants::PORT,
            log_level: LogLevel::Normal,
            cli_colors: true,
            ..Config::default()
        })
}
