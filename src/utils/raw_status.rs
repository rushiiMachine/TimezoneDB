use rocket::http::Status;
use rocket::{Request, Response};
use rocket::response::Responder;

#[derive(Debug, Clone, PartialEq)]
pub struct RawStatus<R>(pub Status, pub R);

impl<'r, 'o: 'r, R: Responder<'r, 'o>> Responder<'r, 'o> for RawStatus<R> {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'o> {
        Response::build()
            .merge(self.1.respond_to(req)?)
            .status(self.0)
            .ok()
    }
}
