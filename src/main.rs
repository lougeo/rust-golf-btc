#[macro_use]
extern crate rocket;

use rocket_db_pools::Database;

mod auth;
mod errors;
mod users;

use crate::auth::security::ApiKey;

#[derive(Database)]
#[database("golf")]
pub struct DBConn(sqlx::PgPool);

#[get("/")]
fn index(api_key: ApiKey) -> String {
    format!("Hi, {:?}!", api_key)
}

#[get("/", rank = 2)]
fn index_anonymous() -> &'static str {
    "Please login"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DBConn::init())
        .mount("/", routes![index, index_anonymous])
        .mount("/auth", auth::api::routes())
        .mount("/user", users::api::routes())
        .register("/", errors::api::catchers())
}
