#[macro_use] extern crate rocket;

mod handlers;
mod config;
mod storage;
mod error;

use crate::config::Config;
use crate::storage::storage::Storage;

use crate::handlers::utils::{health_handler, echo_handler};

#[launch]
async fn rocket() -> _ {
    let config = Config::new();

    rocket::build()
        .configure(rocket::Config::figment().merge(("port", config.api_port)))
        .mount("/", routes![health_handler])
        .mount("/", routes![echo_handler])
}
