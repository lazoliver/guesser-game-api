<<<<<<< HEAD
#[macro_use] extern crate rocket;

mod handlers;

use crate::handlers::utils::{health_handler, echo_handler};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![health_handler])
        .mount("/", routes![echo_handler])
}
=======
fn main() {
    println!("Hello, world!");
}
>>>>>>> master
