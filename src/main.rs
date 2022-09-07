#[macro_use]
extern crate rocket;
#[path = "./context/db_context.rs"]
mod context;

extern crate serde_json;


#[get("/")]
fn index() -> &'static str {
    let test: &str = "Hello, world!";
    return test;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/hi", routes![indexes])
}

#[get("/hi")]
fn indexes() -> &'static str {
    "Test"
}
