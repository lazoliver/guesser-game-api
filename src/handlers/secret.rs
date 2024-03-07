use log::debug;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Response, State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;
use crate::rocket::futures::TryStreamExt;

use crate::Storage;
use crate::{
    error::AppError,
    storage::{
        secret::{NewSecret, SecretEntity},
        storage::AttemptCountRule,
    },
};

#[derive(Serialize)]
pub struct AllSecretsResponse {
    secrets: Vec<Secret>,
}

#[derive(Clone, Deserialize)]
pub struct GuessSecret {
    guess: String,
    username: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Secret {
    pub id: Uuid,
    pub guessed: bool,
    pub clue1: Option<String>,
    pub clue2: Option<String>,
    pub clue3: Option<String>,
    pub guesser: Option<String>,
    pub secret: Option<String>,
}

pub fn process_secret(attempt_rule: AttemptCountRule, entity: SecretEntity) -> Secret {
    Secret {
        id: entity.id,
        guessed: entity.guesser.is_some(),
        clue1: match entity.guess_attempts >= attempt_rule.clue1_attempts {
            true => Some(entity.clue1),
            false => None,
        },
        clue2: match entity.guess_attempts >= attempt_rule.clue2_attempts {
            true => Some(entity.clue2),
            false => None,
        },
        clue3: match entity.guess_attempts >= attempt_rule.clue3_attempts {
            true => Some(entity.clue3),
            false => None,
        },
        secret: entity.guessed_secret,
        guesser: entity.guesser,
    }
}

#[post("/secrets", format = "json", data = "<secret>")]
pub async fn create_secret_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
    secret: Json<NewSecret>,
) -> Result<Json<Secret>, status::Custom<Json<Value>>> {
    let created_secret = storage
        .create_secret(secret.into_inner())
        .await;

    let secret = match created_secret {
        Ok(secret) => secret,
        Err(_) => return Err(status::Custom(Status::InternalServerError, Json(json!({"error": "Internal Server Error."}))))
    };

    let processed_secret = process_secret(*attempt_rule.inner(), secret);

    debug!("Create Secret Handler executed successfully.");

    Ok(Json(processed_secret))
}

#[get("/secrets/<id>")]
pub async fn get_secret_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
    id: &str,
) -> Result<Json<Secret>, status::Custom<Json<Value>>> {
    let secret_id = match Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Err(status::Custom(Status::BadRequest, Json(json!({"error": "Invalid Secret ID."}))))
    };

    let storage_secret = storage.get_secret_entity(secret_id).await;

    let secret = match storage_secret {
        Ok(secret) => secret,
        Err(AppError::NotFound) => {
            return Err(status::Custom(
                Status::NotFound,
                Json(json!({"error": "Secret not found."})),
            ));
        }
        Err(_) => return Err(status::Custom(Status::InternalServerError, Json(json!({"error": "Internal Server Error."})))),
    };

    let processed_secret = process_secret(*attempt_rule.inner(), secret);

    debug!("Get Secret Handler executed successfully.");

    Ok(Json(processed_secret))
}

#[get("/secrets")]
pub async fn get_all_unguessed_secrets_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
) -> Result<Json<AllSecretsResponse>, status::Custom<Json<Value>>> {
    let storage_secrets = storage
        .get_all_unguessed_secrets()
        .await;

    let secret_entities = match storage_secrets {
        Ok(secrets) => secrets,
        Err(_) => return Err(status::Custom(Status::InternalServerError, Json(json!({"error": "Internal Server Error."}))))
    };

    let secrets: Vec<Secret> = secret_entities.into_iter().map(|secret_entity| process_secret(*attempt_rule.inner(), secret_entity)).collect();

    let response = AllSecretsResponse {
        secrets: secrets.clone(),
    };

    debug!("Get All Secrets Handler executed successfully.");

    Ok(Json(response))
}

#[post("/secrets/<id>", format = "json", data = "<guess>")]
pub async fn guess_secret_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
    id: &str,
    guess: Json<GuessSecret>,
) -> Result<Json<Secret>, status::Custom<Json<Value>>> {
    let secret_id = match Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Err(status::Custom(Status::BadRequest, Json(json!({"error": "Invalid Secret ID."}))))
    };

    let guess_secret = storage
        .guess_secret(secret_id, guess.guess.clone(), guess.username.clone())
        .await;

    match guess_secret {
        Ok(secret) => secret,
        Err(AppError::NotFound) => {
            return Err(status::Custom(
                Status::NotFound,
                Json(json!({"error": "Secret not found."})),
            ))
        }
        Err(AppError::AlreadyGuessed) => {
            return Err(status::Custom(
                Status::BadRequest,
                Json(json!({"error": "Secret already guessed."})),
            ))
        }
        Err(_) => return Err(status::Custom(Status::InternalServerError, Json(json!({"error": "Internal Server Error"})))),
    };

    let secret = storage.get_secret_entity(secret_id).await.unwrap();

    let processed_secret = process_secret(*attempt_rule.inner(), secret);

    debug!("Guess Secret Handler executed successfully.");

    Ok(Json(processed_secret))
}