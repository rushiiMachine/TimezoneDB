use std::path::PathBuf;

use rocket::fairing::AdHoc;

#[cfg(not(debug_assertions))]
use {
    include_dir::{Dir, include_dir},
    rocket::http::ContentType,
    crate::utils::cache_control::CacheControl,
};

#[cfg(debug_assertions)]
use rocket::response::Redirect;

#[cfg(not(debug_assertions))]
static REACT_BUILD: Dir = include_dir!("$CARGO_MANIFEST_DIR/build");

#[cfg(not(debug_assertions))]
#[get("/")]
async fn index() -> CacheControl<(ContentType, &'static str)> {
    let file = REACT_BUILD.get_file("index.html").unwrap();
    let contents = file.contents_utf8().unwrap();
    CacheControl((ContentType::HTML, contents))
}

#[cfg(not(debug_assertions))]
#[get("/<path..>", rank = 999)]
async fn files(path: PathBuf) -> CacheControl<Option<(ContentType, &'static [u8])>> {
    let file = REACT_BUILD
        .get_file(&path.to_string_lossy().to_string())
        .map(|file| file.contents());

    let option = match file {
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
    };
    CacheControl(option)
}


#[cfg(debug_assertions)]
#[get("/")]
async fn debug_index() -> Redirect {
    Redirect::to("http://localhost:3000")
}

#[cfg(debug_assertions)]
#[get("/<path..>", rank = 999)]
async fn debug_redirect(path: PathBuf) -> Redirect {
    Redirect::to(format!("http://localhost:3000/{}", path.to_string_lossy().to_string()))
}

pub fn routes() -> AdHoc {
    AdHoc::on_ignite("Root Routing", |rocket| async {
        #[cfg(debug_assertions)]
        {
            rocket.mount(
                "/",
                routes![debug_index, debug_redirect],
            )
        }
        #[cfg(not(debug_assertions))]
        {
            rocket.mount(
                "/",
                routes![index, files],
            )
        }
    })
}
