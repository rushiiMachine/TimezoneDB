use either::{Either, Left, Right};
use rocket::fairing::AdHoc;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::http::uri::{Reference, Uri};
use rocket::response::Redirect;
use rocket::time::{Duration, OffsetDateTime};
use rocket_db_pools::Connection;

use crate::{constants, logic};
use crate::database::Db;
use crate::utils::discord::{DiscordApiError, DiscordApiErrorData};

#[get("/")]
async fn redirect() -> Redirect {
    let uri = Uri::parse::<Reference>(&*constants::DISCORD_OAUTH_URL)
        .expect("failed to construct discord oauth uri");

    Redirect::to(uri)
}

#[get("/?error=access_denied")]
async fn auth_denied() -> Redirect {
    Redirect::to("/")
}

#[get("/?<code>")]
async fn code(code: String, cookies: &CookieJar<'_>, db: Connection<Db>) -> Either<Redirect, Status> {
    match logic::auth::login_user(code, db).await {
        Ok(jwt_token) => {
            cookies.add(Cookie::build(("loginInfo", jwt_token))
                .secure(true)
                .http_only(true)
                .same_site(SameSite::Lax)
                .expires(OffsetDateTime::now_utc() + Duration::days(30)));

            let host = Uri::parse::<Reference>(&*constants::HOST)
                .expect("invalid HOST env var");
            Left(Redirect::to(host))
        }
        Err(err) => {
            match err.downcast_ref::<DiscordApiError>() {
                Some(DiscordApiError::ApiError(DiscordApiErrorData { error, .. })) if error == "invalid_grant" => {
                    return Right(Status::BadRequest);
                }
                _ => {}
            };

            println!("failed to auth user {:?}", err);
            Right(Status::InternalServerError)
        }
    }
}

#[get("/logout")]
async fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove(Cookie::from("loginInfo"));
    Redirect::to("/")
}

pub fn routes() -> AdHoc {
    AdHoc::on_ignite("Auth Routing", |rocket| async {
        rocket.mount(
            "/api/auth",
            routes![
                redirect,
                auth_denied,
                code,
                logout,
            ],
        )
    })
}
