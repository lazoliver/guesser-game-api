#[macro_use]
extern crate rocket;

use uuid::uuid;

use std::time::SystemTime;

use env_logger::Env;

mod config;
mod error;
mod handlers;
mod storage;

use crate::config::Config;
use crate::config::ReleaseMode;
use crate::storage::secret::NewSecret;
use crate::storage::storage::{AttemptCountRule, Storage};

use crate::handlers::utils::{echo_handler, full_health_handler, health_handler};

#[launch]
async fn rocket() -> _ {
    let config = Config::new();

    let attempts = AttemptCountRule {
        clue1_attempts: config.clue1_attempts,
        clue2_attempts: config.clue2_attempts,
        clue3_attempts: config.clue3_attempts,
    };

    let storage = Storage::new(config.mongo_uri)
        .await
        .expect("Error to connecting database");

    // let new_secret = NewSecret {
    //     secret: "Futebol".to_string(),
    //     clue1: "Pés".to_string(),
    //     clue2: "22".to_string(),
    //     clue3: "Bola".to_string(),
    // };

    // let result = storage
    //     .create_secret(new_secret)
    //     .await
    //     .expect("Deu ruim fi!");

    // println!("{:?}", result);

    // let secret_id = uuid!("6697f8b4-8abe-467c-aa2a-867791ca1dc3");

    // let guess = "Futebol".to_string();

    // let username = "Wilian".to_string();

    // let sec = storage.guess_secret(secret_id, guess, username).await;

    // println!("{:?}", sec);

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
        .manage(attempts)
        .configure(rocket::Config::figment().merge(("port", config.api_port)))
        .mount(
            "/",
            routes![health_handler, echo_handler, full_health_handler],
        )
}
