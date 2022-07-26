use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;

use crate::utils::jwt::JwtData;
use crate::utils::snowflake::Snowflake;

#[get("/<id>")]
async fn get_user(id: Snowflake) -> Value {
    // TODO: fetch user from db and calculate offset

    json!({"userId": id, "timezone": "-7"})
}

#[get("/")]
async fn get_current_user(user: JwtData) -> Redirect {
    Redirect::to(user.user_id.to_string())
}

#[get("/<id>/exists")]
async fn exists_user(id: Snowflake) -> Status {
    // TODO: check db if user exists

    Status::Ok // or 404 if not found
}

#[delete("/")]
async fn delete_user(user: JwtData) -> Status {
    // TODO: delete record from db

    Status::Ok
}

pub fn routes() -> AdHoc {
    AdHoc::on_ignite("User Routing", |rocket| async {
        rocket.mount(
            "/api/user",
            routes![
                get_user,
                get_current_user,
                exists_user,
                delete_user
            ],
        )
    })
}
