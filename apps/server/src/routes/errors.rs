use rocket::{Request, catch};
use rocket::http::Status;

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}