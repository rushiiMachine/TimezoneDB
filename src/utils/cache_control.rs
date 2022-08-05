use rocket::{Request, Response};
use rocket::response::Responder;

#[derive(Debug, Clone, PartialEq)]
pub struct CacheControl<R>(pub R);

impl<'r, 'o: 'r, R: Responder<'r, 'o>> Responder<'r, 'o> for CacheControl<R> {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'o> {
        Response::build()
            .merge(self.0.respond_to(req)?)
            .raw_header("Cache-Control", "public, max-age=86400") // 24h (24*60*60)
            .ok()
    }
}
