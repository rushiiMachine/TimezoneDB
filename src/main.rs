#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use include_dir::{Dir, include_dir};
use rocket::{Build, Config, Rocket};
use rocket::http::ContentType;
use rocket::response::content::RawHtml;

use crate::utils::jwt::JwtData;

mod constants;
mod routes;
mod utils;
mod logic;
mod database;

static REACT_BUILD: Dir = include_dir!("$CARGO_MANIFEST_DIR/build");

#[get("/")]
async fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../build/index.html"))
}

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
