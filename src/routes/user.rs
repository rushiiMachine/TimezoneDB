use either::{Either, Left, Right};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket_db_pools::Connection;

use crate::database::Db;
use crate::logic;
use crate::utils::jwt::JwtData;
use crate::utils::snowflake::Snowflake;

#[get("/")]
async fn get_current_user(user: JwtData) -> Redirect {
    Redirect::to(user.user_id.to_string())
}

#[delete("/")]
async fn delete_user(user: JwtData, db: Connection<Db>) -> Status {
    let status = logic::user::delete_user(*user.user_id, db);

    match status.await {
        true => Status::Ok,
        false => Status::InternalServerError,
    }
}

#[get("/<id>")]
async fn get_user(id: Snowflake, db: Connection<Db>) -> Either<Status, Value> {
    let user = logic::user::fetch_user(id, db);

    match user.await {
        None => Left(Status::NotFound),
        Some(user) => {
            let data = json!({
                "userId": id,
                "timezone": user.offset, // calculate offset if timezone != null
                "timezoneId": user.timezone,
            });
            Right(data)
        }
    }
}

#[get("/<id>/exists")]
async fn exists_user(id: Snowflake, db: Connection<Db>) -> Status {
    let exists = logic::user::exists_user(id, db);

    match exists.await {
        true => Status::Ok,
        false => Status::NotFound,
    }
}

pub fn routes() -> AdHoc {
    AdHoc::on_ignite("User Routing", |rocket| async {
        rocket.mount(
            "/api/user",
            routes![
                get_user,
                get_current_user,
                exists_user,
                delete_user,
            ],
        )
    })
}
