use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use crate::Snowflake;

#[get("/<id>")]
async fn get(id: Snowflake) -> Value {
    json!({"userId": id, "timezone": "-7"})
}

#[get("/<id>/exists")]
async fn exists(id: Snowflake) -> Status {
    Status::Ok
}

#[delete("/")]
async fn delete() -> Redirect {
    Redirect::to(uri!("/"))
}

pub fn routes() -> AdHoc {
    AdHoc::on_ignite("user routes", |rocket| async {
        rocket.mount("/api/user", routes![get, exists, delete])
    })
}
