#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello from lunaire.rs"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}