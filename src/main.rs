#[macro_use] extern crate rocket;

use env_logger::Env;

mod handlers;
mod config;
mod storage;
mod error;

use crate::config::Config;
use crate::config::ReleaseMode;

use crate::handlers::utils::{health_handler, echo_handler};

#[launch]
async fn rocket() -> _ {
    let config = Config::new();

    let default_level = match config.release_mode {
        ReleaseMode::Dev => "debug",
        ReleaseMode::Prod => "info"
    };

    let env = Env::default().default_filter_or(default_level);
    env_logger::init_from_env(env);

    let startup_message = format!("Server is running on http://localhost:{}", config.api_port);
    info!("{}", startup_message);

    rocket::build()
        .configure(rocket::Config::figment().merge(("port", config.api_port)))
        .mount("/", routes![health_handler])
        .mount("/", routes![echo_handler])
}
