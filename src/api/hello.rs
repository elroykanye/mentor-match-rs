#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, Rocket"
}