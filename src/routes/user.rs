use either::{Either, Left, Right};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket_db_pools::Connection;

use crate::database::Db;
use crate::logic;
use crate::logic::user::UserUpdateData;
use crate::utils::jwt::JwtData;
use crate::utils::snowflake::Snowflake;

#[get("/")]
async fn get_current_user(user: JwtData) -> Redirect {
    Redirect::to(user.user_id.to_string())
}

#[delete("/")]
async fn delete_user(user: JwtData, mut db: Connection<Db>) -> Status {
    let status = logic::user::delete_user(*user.user_id, &mut *db);

    match status.await {
        true => Status::Ok,
        false => Status::InternalServerError,
    }
}

#[put("/", data = "<data>", format = "application/json")]
async fn update_user(user: JwtData, data: Json<UserUpdateData>, mut db: Connection<Db>) -> Status {
    let add_status = logic::user::add_user(&user, &mut *db).await;
    let update_status = logic::user::update_user(&user, data.0, &mut *db).await;

    match add_status && update_status {
        true => Status::Ok,
        false => Status::InternalServerError,
    }
}

#[get("/<id>")]
async fn get_user(id: Snowflake, mut db: Connection<Db>) -> Either<Status, Value> {
    let user = logic::user::fetch_user(id, &mut *db);

    match user.await {
        None => Left(Status::NotFound),
        Some(user) if user.offset.is_none() && user.timezone.is_none() =>
            Left(Status::NotFound),
        Some(user) => {
            let data = json!({
                "userId": id,
                "timezone": "+0", // calculate offset if timezone != null
                "timezoneId": user.timezone,
            });
            Right(data)
        }
    }
}

#[get("/<id>/exists")]
async fn exists_user(id: Snowflake, mut db: Connection<Db>) -> Status {
    let exists = logic::user::exists_user(id, &mut *db);

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
                get_current_user,
                delete_user,
                update_user,
                get_user,
                exists_user,
            ],
        )
    })
}
