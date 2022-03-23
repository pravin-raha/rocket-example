#[macro_use]
extern crate rocket;

use rocket::Error;
use rocket_example;

#[rocket::main]
async fn main() -> Result<(), Error> {
    rocket::build()
        .mount("/", routes![rocket_example::routes::users::create_user])
        .launch()
        .await
}

