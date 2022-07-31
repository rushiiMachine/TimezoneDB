use rocket::fairing::AdHoc;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

use crate::utils::jwt::JwtData;

mod user;
mod auth;
mod catchers;
mod root;

#[derive(Debug, Serialize, Deserialize)]
struct ApiStatusResponse<'a> {
    status: &'a str,
    logged_in: bool,
}

#[get("/")]
fn status(user: Option<JwtData>) -> Json<ApiStatusResponse<'static>> {
    Json(ApiStatusResponse {
        status: "online",
        logged_in: user.is_some(),
    })
}

pub fn setup() -> AdHoc {
    AdHoc::on_ignite("API Routing", |rocket| async {
        rocket
            .mount("/api", routes![status])
            .attach(user::routes())
            .attach(auth::routes())
            .attach(root::routes())
            .attach(catchers::setup())
    })
}
