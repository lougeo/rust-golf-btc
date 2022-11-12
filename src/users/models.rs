use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use sqlx::{postgres::PgRow, Row};

use crate::DBConn;

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseUser {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

impl BaseUser {
    pub fn create_user() -> BaseUser {
        BaseUser {
            id: 1,
            email: "test@test.com".to_string(),
            password: "test".to_string(),
            first_name: "test".to_string(),
            last_name: "test".to_string(),
        }
    }

    pub async fn get_users(mut db: Connection<DBConn>) -> Result<Vec<BaseUser>, sqlx::Error> {
        match sqlx::query("SELECT * FROM users")
            .map(|row: PgRow| BaseUser {
                id: row.get("id"),
                email: row.get("email"),
                password: row.get("password"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
            })
            .fetch_all(&mut *db)
            .await
        {
            Ok(users) => Ok(users),
            Err(e) => Err(e),
        }
    }
}
