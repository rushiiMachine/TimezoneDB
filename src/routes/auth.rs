use rocket::fairing::AdHoc;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::http::uri::{Reference, Uri};
use rocket::response::Redirect;
use rocket::time::{Duration, OffsetDateTime};

use crate::{constants, logic};

#[get("/")]
async fn redirect() -> Redirect {
    let uri = Uri::parse::<Reference>(&*constants::DISCORD_OAUTH_URL)
        .expect("failed to construct discord oauth uri");

    Redirect::to(uri)
}

#[get("/?<code>")]
async fn code(code: String, cookies: &CookieJar<'_>) -> Redirect {
    match logic::auth::login_user(code).await {
        Ok(jwt_token) => {
            let cookie = Cookie::build("loginInfo", jwt_token)
                .secure(true)
                .http_only(true)
                .same_site(SameSite::Strict)
                .expires(OffsetDateTime::now_utc() + Duration::days(30))
                .finish();

            cookies.add(cookie);
        }
        Err(err) => {
            // TODO: redirect with error message
            println!("{:?}", err);
        }
    }

    let host = Uri::parse::<Reference>(&*constants::HOST)
        .expect("invalid HOST env var");
    Redirect::to(host)
}

pub fn routes() -> AdHoc {
    AdHoc::on_ignite("Auth Routing", |rocket| async {
        rocket.mount("/api/auth", routes![redirect, code])
    })
}
