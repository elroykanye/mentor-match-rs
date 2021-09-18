mod db;
mod model;
mod service;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, World"
}

//noinspection ALL
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
/*
fn main() {
    println!("Hello, world!");
}
*/
fn main() {}