use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::Request;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    error: String,
}

macro_rules! catcher {
    ($code:literal, $name:ident) => {
        #[catch($code)]
        pub fn $name() -> Json<ApiError> {
            Json(ApiError { error: stringify!($name).to_string() })
        }
    };
}

#[catch(default)]
fn default(status: Status, _: &Request<'_>) -> Json<ApiError> {
    Json(ApiError { error: format!("Unknown error {}", status.code) })
}

catcher!(400, invalid_request);
catcher!(401, unauthorized);
catcher!(404, not_found);
catcher!(500, internal_error);

pub fn setup() -> AdHoc {
    AdHoc::on_ignite("API Catchers", |rocket| async {
        rocket.register(
            "/api",
            catchers![
                default,
                invalid_request,
                internal_error,
                not_found,
                unauthorized,
            ],
        )
    })
}
