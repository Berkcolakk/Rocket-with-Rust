#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/hi", routes![indexes])
}