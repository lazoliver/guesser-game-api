#[macro_use] extern crate rocket;

mod handlers;
mod config;

use crate::config::Config;

use crate::handlers::utils::{health_handler, echo_handler};

#[launch]
fn rocket() -> _ {
    let config = Config::new();

    rocket::build()
        .configure(rocket::Config::figment().merge(("port", config.api_port)))
        .mount("/", routes![health_handler])
        .mount("/", routes![echo_handler])
}