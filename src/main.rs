#[macro_use]
extern crate rocket;

use rocket::Config;
use rocket::serde::json::Value;
use rocket::serde::json::serde_json::json;

use crate::utils::jwt::JwtData;

mod constants;
mod routes;
mod utils;
mod logic;
mod database;

#[get("/")]
pub async fn index(user: Option<JwtData>) -> Value {
    // TODO: templated dashboard

    json!({"status": "online", "logged_in": user.is_some() })
}

#[launch]
fn rocket() -> _ {
    let figment = Config::figment()
        .merge(("port", *constants::PORT))
        .merge(("databases.main.url", "./database.db"));

    rocket::custom(figment)
        .mount("/", routes![index])
        .attach(database::setup())
        .attach(routes::catchers::setup())
        .attach(routes::user::routes())
        .attach(routes::auth::routes())
}
