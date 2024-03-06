use log::debug;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Response, State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::Storage;
use crate::{
    error::AppError,
    storage::{
        secret::{NewSecret, Secret, SecretEntity},
        storage::AttemptCountRule,
    },
};

#[derive(Serialize)]
pub struct SecretResponse {
    msg: String,
}

#[derive(Serialize)]
pub struct AllSecretsResponse {
    secrets: Vec<Secret>,
}

#[derive(Clone, Deserialize)]
pub struct GuessSecret {
    guess: String,
    username: String,
}

#[post("/secret", format = "json", data = "<secret>")]
pub async fn create_secret_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
    secret: Json<NewSecret>,
) -> Result<Json<SecretResponse>, status::Custom<Json<Value>>> {
    let secret = storage
        .create_secret(*attempt_rule.inner(), secret.into_inner())
        .await
        .unwrap();

    let response = SecretResponse {
        msg: String::from("Secret successfully created."),
    };

    debug!("Create Secret Handler executed successfully.");

    Ok(Json(response))
}

#[get("/secret/<id>")]
pub async fn get_secret_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
    id: &str,
) -> Result<Json<Secret>, status::Custom<Json<Value>>> {
    let plain_id = Uuid::parse_str(id).unwrap();

    let secret_result = storage.get_secret_entity(plain_id).await;

    let secret = match secret_result {
        Ok(secret) => secret,
        Err(AppError::NotFound) => {
            return Err(status::Custom(
                Status::NotFound,
                Json(json!({"error": "Secret not found."})),
            ));
        }
        _ => todo!(),
    };

    let processed_secret = storage.process_secret(*attempt_rule.inner(), secret);

    debug!("Get Secret Handler executed successfully.");

    Ok(Json(processed_secret))
}

#[get("/secret/all")]
pub async fn get_all_secrets_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
) -> Json<AllSecretsResponse> {
    let secrets = storage
        .get_all_secrets(*attempt_rule.inner())
        .await
        .unwrap();

    let response = AllSecretsResponse {
        secrets: secrets.clone(),
    };

    debug!("Get All Secrets Handler executed successfully.");

    Json(response)
}

#[post("/secret/<id>", format = "json", data = "<guess>")]
pub async fn guess_secret_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
    id: &str,
    guess: Json<GuessSecret>,
) -> Result<Json<Secret>, status::Custom<Json<Value>>> {
    let plain_id = Uuid::parse_str(id).unwrap();

    let guess_secret = storage
        .guess_secret(plain_id, guess.guess.clone(), guess.username.clone())
        .await;

    let guessed_secret = match guess_secret {
        Ok(secret) => secret,
        Err(AppError::NotFound) => {
            return Err(status::Custom(
                Status::NotFound,
                Json(json!({"msg": "Secret not found."})),
            ))
        }
        Err(AppError::AlreadyGuessed) => {
            return Err(status::Custom(
                Status::Conflict,
                Json(json!({"msg": "Secret already guessed."})),
            ))
        }
        _ => todo!(),
    };

    let secret = storage.get_secret_entity(plain_id).await.unwrap();

    let processed_secret = storage.process_secret(*attempt_rule.inner(), secret);

    debug!("Guess Secret Handler executed successfully.");

    Ok(Json(processed_secret))
}
