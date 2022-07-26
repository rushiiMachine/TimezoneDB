#[macro_use]
extern crate rocket;

use std::net::{IpAddr, Ipv4Addr};

use rocket::Config;
use rocket::config::LogLevel;
use rocket::serde::json::Value;
use rocket::serde::json::serde_json::json;

use crate::utils::jwt::JwtData;

mod constants;
mod routes;
mod utils;
mod logic;

#[get("/")]
async fn index(user: Option<JwtData>) -> Value {
    // TODO: templated dashboard

    json!({"status": "online", "logged_in": user.is_some() })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(routes::catchers::setup())
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
