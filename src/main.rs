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

    let database = Storage::new().await.unwrap();

    let secret_word = "Wilian".to_string();

    database.create_secret(secret_word.clone()).await.unwrap();

    rocket::build()
        .configure(rocket::Config::figment().merge(("port", config.api_port)))
        .mount("/", routes![health_handler])
        .mount("/", routes![echo_handler])
}
