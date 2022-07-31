#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use rocket::{Build, Config, Rocket};
use rocket::response::Redirect;

#[cfg(debug_assertions)]
use {
    include_dir::{Dir, include_dir},
    rocket::http::ContentType,
    rocket::response::content::RawHtml,
};

use crate::utils::jwt::JwtData;

mod constants;
mod routes;
mod utils;
mod logic;
mod database;

#[cfg(debug_assertions)]
#[get("/")]
async fn index() -> Redirect {
    Redirect::to("http://localhost:3000")
}

#[cfg(debug_assertions)]
#[get("/<path..>", rank = 2)]
async fn files(path: PathBuf) -> Redirect {
    Redirect::to(format!("http://localhost:3000/{}", path.to_string_lossy().to_string()))
}

#[cfg(not(debug_assertions))]
static REACT_BUILD: Dir = include_dir!("$CARGO_MANIFEST_DIR/build");

#[cfg(not(debug_assertions))]
#[get("/")]
async fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../build/index.html"))
}

#[cfg(not(debug_assertions))]
#[get("/<path..>", rank = 2)]
async fn files(path: PathBuf) -> Option<(ContentType, &'static [u8])> {
    let file = REACT_BUILD
        .get_file(&path.to_string_lossy().to_string())
        .map(|file| file.contents());

    match file {
        None => None,
        Some(bytes) => {
            let ext = &path
                .extension()
                .map(|ext| ext.to_string_lossy())
                .and_then(|ext| ContentType::from_extension(&*ext));

            match ext {
                Some(content_type) =>
                    Some((content_type.clone(), bytes)),
                None =>
                    Some((ContentType::Binary, bytes)),
            }
        }
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    let figment = Config::figment()
        .merge(("port", *constants::PORT))
        .merge(("databases.main.url", "./database.db"));

    rocket::custom(figment)
        .mount("/", routes![index, files])
        .attach(database::setup())
        .attach(routes::setup())
}
