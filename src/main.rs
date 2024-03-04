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
use crate::storage::storage::{AttemptCountRule, Storage};

use crate::handlers::secret::{
    create_secret_handler, get_all_secrets_handler, get_secret_handler, guess_secret_handler,
};
use crate::handlers::utils::{echo_handler, full_health_handler, health_handler};

#[launch]
async fn rocket() -> _ {
    let config = Config::new();

    let attempt_rule = AttemptCountRule {
        clue1_attempts: config.clue1_attempts,
        clue2_attempts: config.clue2_attempts,
        clue3_attempts: config.clue3_attempts,
    };

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
        .manage(attempt_rule)
        .manage(SystemTime::now())
        .configure(rocket::Config::figment().merge(("port", config.api_port)))
        .mount(
            "/",
            routes![health_handler, echo_handler, full_health_handler],
        )
        .mount(
            "/",
            routes![
                create_secret_handler,
                get_secret_handler,
                get_all_secrets_handler,
                guess_secret_handler
            ],
        )
}
