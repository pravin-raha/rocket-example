use rocket::post;
use rocket::serde::json::Json;

use crate::models::users;

#[post("/user", format = "json", data = "<user>")]
pub fn create_user(user: Json<users::User>) -> Json<users::User> {
    user
}

