use rocket::serde::{json::Json, Serialize};

use rocket::http::Status;
use rocket::Request;

#[derive(Serialize)]
struct Error {
    pub status: u16,
    pub description: String,
}

#[catch(401)]
fn permission_denied(status: Status, req: &Request) -> Json<Error> {
    Json(Error {
        status: status.code,
        description: req.uri().to_string(),
    })
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![permission_denied]
}
