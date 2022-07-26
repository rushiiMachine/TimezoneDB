use reqwest::Client;
use rocket::fairing::AdHoc;
use rocket::http::{Cookie, CookieJar, Status, uri};
use rocket::http::uri::Uri;
use rocket::http::uri::Uri::Reference;
use rocket::response::{Flash, Redirect, status};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;

use crate::Snowflake;

#[get("/?<code>")]
async fn authenticate(code: Option<String>) -> Value {


    json!({})
}

pub fn routes() -> AdHoc {
    AdHoc::on_ignite("auth routes", |rocket| async {
        rocket.mount("/api/auth", routes![authenticate])
    })
}
