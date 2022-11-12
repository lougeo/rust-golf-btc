use crate::auth::security::ApiKey;

// #[post("/login", data = "<login>")]
// fn login(login: Form<Login<'_>>) -> Json<T> {
//     if login.username == "Sergio" && login.password == "password" {
//         // Create jws token
//         let test_data = r#"{"test": "123"}"#;
//         Json(test_data)
//     } else {
//         Err(Json("Invalid username/password."))
//     }
// }

#[get("/logout")]
fn logout(api_key: ApiKey) -> &'static str {
    // Invalidate token
    "Logged out."
}

pub fn routes() -> Vec<rocket::Route> {
    routes![logout]
}
