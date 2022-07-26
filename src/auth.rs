use rocket::fairing::AdHoc;
use rocket::http::uri::{Reference, Uri};
use rocket::response::Redirect;

use crate::{constants, discord_api};

#[get("/")]
async fn redirect() -> Redirect {
    let uri = Uri::parse::<Reference>(&*constants::DISCORD_OAUTH_URL)
        .expect("failed to construct discord oauth uri");

    Redirect::to(uri)
}

#[get("/?<code>")]
async fn code(code: String) -> Redirect {
    let oauth_data = discord_api::complete_oauth_flow(code)
        .await
        .map_err(|err| println!("{:?}", err))
        .ok();

    if let Some(oauth_data) = oauth_data {
        let auth = format!("{0} {1}", oauth_data.token_type, oauth_data.access_token);
        let user = discord_api::get_current_user(&auth)
            .await
            .map_err(|err| println!("{:?}", err))
            .ok();

        if let Some(user) = user {
            println!("{:?}", user);
        }
    }

    let host = Uri::parse::<Reference>(&*constants::HOST)
        .expect("invalid HOST env var");
    Redirect::to(host)
}

pub fn routes() -> AdHoc {
    AdHoc::on_ignite("auth routes", |rocket| async {
        rocket.mount("/api/auth", routes![redirect, code])
    })
}
