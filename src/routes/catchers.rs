use rocket::fairing::AdHoc;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError<'a> {
    error: &'a str,
}

macro_rules! catcher {
    ($code:literal, $name:ident) => {
        #[catch($code)]
        fn $name() -> Json<ApiError<'static>> {
            Json(ApiError { error: stringify!($name) })
        }
    };
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
                invalid_request,
                internal_error,
                not_found,
                unauthorized,
            ],
        )
    })
}
