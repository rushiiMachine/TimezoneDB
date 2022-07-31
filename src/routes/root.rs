use std::path::PathBuf;

use include_dir::{Dir, include_dir};
use rocket::fairing::AdHoc;
use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;

static REACT_BUILD: Dir = include_dir!("$CARGO_MANIFEST_DIR/build");

#[get("/")]
async fn index() -> RawHtml<&'static str> {
    RawHtml(REACT_BUILD.get_file("index.html").unwrap().contents_utf8().unwrap())
}

#[get("/<path..>", rank = 999)]
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


#[get("/")]
async fn debug_index() -> Redirect {
    Redirect::to("http://localhost:3000")
}

#[get("/<path..>", rank = 999)]
async fn debug_redirect(path: PathBuf) -> Redirect {
    Redirect::to(format!("http://localhost:3000/{}", path.to_string_lossy().to_string()))
}

pub fn routes() -> AdHoc {
    AdHoc::on_ignite("Root Routing", |rocket| async {
        if cfg!(debug_assertions) {
            rocket.mount(
                "/",
                routes![debug_index, debug_redirect],
            )
        } else {
            rocket.mount(
                "/",
                routes![index, files],
            )
        }
    })
}
