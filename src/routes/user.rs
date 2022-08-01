use either::{Either, Left, Right};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::database::Db;
use crate::logic;
use crate::logic::user::UserUpdateData;
use crate::utils::jwt::JwtData;
use crate::utils::snowflake::{ApiSnowflake, Snowflake};

#[get("/")]
async fn get_current_user(user: JwtData) -> Redirect {
    Redirect::to(format!("/api/user/{0}", *user.user_id))
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

#[derive(Serialize, Deserialize, Debug)]
struct GetUserData {
    #[serde(rename = "userId")]
    id: ApiSnowflake,
    #[serde(rename = "timezoneId")]
    timezone_id: Option<String>,
    timezone: String,
}

#[get("/<id>")]
async fn get_user(id: Snowflake, mut db: Connection<Db>) -> Either<Status, Json<GetUserData>> {
    let user = logic::user::fetch_user(id, &mut *db);

    match user.await {
        None => Left(Status::NotFound),
        Some(user) if user.timezone.is_none() =>
            Left(Status::NotFound),
        Some(user) => {
            let data = GetUserData {
                id: ApiSnowflake(user.id),
                timezone: logic::user::calculate_tz_offset(&user.timezone),
                timezone_id: user.timezone,
            };
            Right(Json(data))
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
