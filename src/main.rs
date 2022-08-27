#[macro_use]
extern crate rocket;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("golf")]
struct DBConn(sqlx::PgPool);

#[get("/users/<id>")]
async fn users_detail(mut db: Connection<DBConn>, id: i64) -> Option<String> {
    sqlx::query("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *db)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DBConn::init())
        .mount("/", routes![index, users_detail])
}
