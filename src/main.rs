#[macro_use]
extern crate rocket;

mod user;
mod auth;
mod discord_auth;

pub type Snowflake = u64;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(user::routes())
        .attach(auth::routes())
}
