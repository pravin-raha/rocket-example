use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
}
