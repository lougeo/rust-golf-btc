use rocket::serde::json::Json;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::Connection;

use crate::users::models::BaseUser;
use crate::DBConn;

#[get("/test")]
fn test() -> Json<BaseUser> {
    let test_user = BaseUser::create_user();
    Json(test_user)
}

#[get("/users")]
async fn user_list(db: Connection<DBConn>) -> Result<Json<Vec<BaseUser>>, Json<String>> {
    match BaseUser::get_users(db).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(Json(e.to_string())),
    }
}

#[get("/users/<id>")]
async fn user_detail(mut db: Connection<DBConn>, id: i64) -> Option<String> {
    sqlx::query("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *db)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![test, user_list, user_detail]
}
