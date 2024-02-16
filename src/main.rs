#[macro_use]
extern crate rocket;
use env_logger::Env;
use std::time::SystemTime;

mod config;
mod error;
mod handlers;
mod storage;

use crate::config::Config;
use crate::config::ReleaseMode;
use crate::storage::storage::Storage;

use crate::handlers::utils::{echo_handler, full_health_handler, health_handler};

#[launch]
async fn rocket() -> _ {
    let config = Config::new();

    let storage = Storage::new(config.mongo_uri)
        .await
        .expect("Error to connecting database");

    let default_level = match config.release_mode {
        ReleaseMode::Dev => "debug",
        ReleaseMode::Prod => "info",
    };

    let env = Env::default().default_filter_or(default_level);
    env_logger::init_from_env(env);

    let startup_message = format!("Server is running on http://localhost:{}", config.api_port);
    info!("{}", startup_message);

    rocket::build()
        .manage(storage)
        .manage(SystemTime::now())
        .configure(rocket::Config::figment().merge(("port", config.api_port)))
        .mount(
            "/",
            routes![health_handler, echo_handler, full_health_handler],
        )
}
