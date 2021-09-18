mod api;

use crate::api::hello::hello;
use crate::api::mentor_con::new_mentor;
use crate::api::mentor_con::get_all_mentors;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, World"
}

//noinspection ALL
#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![index])
        .mount("/mentor/new", routes![new_mentor])
        .mount("/mentor/new", routes![new_mentor])
        .mount("/mentor/all", routes![get_all_mentors])
        .launch()
        .await;
}