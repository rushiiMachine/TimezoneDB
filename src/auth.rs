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
    match discord_api::complete_oauth_flow(code).await {
        Ok(data) => {
            let authorization = format!("{0} {1}", data.token_type, data.access_token);
            let user = discord_api::get_current_user(&authorization);

            match user.await {
                Ok(user) => {

                }
                Err(err) => {
                    println!("{:?}", err)
                }
            }
            println!("{:?}", data);
        }
        Err(err) => {
            println!("{:?}", err)
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
