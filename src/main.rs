#[macro_use] extern crate rocket;

use std::time::SystemTime;

use env_logger::Env;

mod handlers;
mod config;
mod storage;
mod error;

use crate::config::Config;
use crate::storage::storage::{Storage, AttemptCountRule};
use crate::storage::secret::NewSecret;
use crate::config::ReleaseMode;

use crate::handlers::utils::{health_handler, echo_handler, full_health_handler};

#[launch]
async fn rocket() -> _ {
    let config = Config::new();

    let attempts = AttemptCountRule {
        clue1_attempts: config.clue1_attempts,
        clue2_attempts: config.clue2_attempts,
        clue3_attempts: config.clue3_attempts
    };

    let storage = Storage::new(config.mongo_uri).await.expect("Error to connecting database");

    let new_secret = NewSecret {
        secret: "Futebol".to_string(),
        clue1: "Pés".to_string(),
        clue2: "22".to_string(),
        clue3: "Bola".to_string()
    };

    let result = storage.create_secret(new_secret).await.expect("Deu ruim fi!");

    println!("{:?}", result);

    let default_level = match config.release_mode {
        ReleaseMode::Dev => "debug",
        ReleaseMode::Prod => "info"
    };

    let env = Env::default().default_filter_or(default_level);
    env_logger::init_from_env(env);

    let startup_message = format!("Server is running on http://localhost:{}", config.api_port);
    info!("{}", startup_message);

    rocket::build()
        .manage(storage)
        .manage(SystemTime::now())
        .manage(attempts)
        .configure(rocket::Config::figment().merge(("port", config.api_port)))
        .mount("/", routes![health_handler, echo_handler, full_health_handler])
}
