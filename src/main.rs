mod controller;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![controller::test_controller::index, controller::test_controller::hello_name])
}