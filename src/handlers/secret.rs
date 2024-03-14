use log::debug;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::Storage;
use crate::{
    error::AppError,
    storage::secret::{NewSecret, SecretEntity},
};

#[derive(Copy, Clone)]
pub struct AttemptCountRule {
    pub clue1_attempts: u16,
    pub clue2_attempts: u16,
    pub clue3_attempts: u16,
}

#[derive(Clone, Deserialize)]
pub struct GuessSecretRequest {
    guess: String,
    username: String,
}

#[derive(Serialize)]
pub struct AllSecretsResponse {
    secrets: Vec<Secret>
}

#[derive(Clone, Debug, Serialize)]
pub struct Secret {
    pub id: Uuid,
    pub guessed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clue1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clue2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clue3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guesser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    let secret_data = NewSecret {
        secret: secret.secret.to_lowercase(),
        clue1: secret.clue1.to_lowercase(),
        clue2: secret.clue2.to_lowercase(),
        clue3: secret.clue3.to_lowercase(),
    };

    print!("{:?}", secret_data.clone());

    let created_secret = storage.create_secret(secret_data).await;

    let secret = match created_secret {
        Ok(secret) => secret,
        Err(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                Json(json!({"error": "Failed to create secret. Please try again later."})),
            ))
        }
    };

    let processed_secret = process_secret(*attempt_rule.inner(), secret);

    debug!(
        "Create Secret Handler executed successfully and returned a {:?}.",
        processed_secret
    );

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
        Err(_) => {
            return Err(status::Custom(
                Status::BadRequest,
                Json(json!({"error": "Invalid Secret ID."})),
            ))
        }
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
        Err(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                Json(json!({"error": "Internal Server Error."})),
            ))
        }
    };

    let processed_secret = process_secret(*attempt_rule.inner(), secret);

    debug!(
        "Get Secret Handler executed successfully and returned a {:?}.",
        processed_secret
    );

    Ok(Json(processed_secret))
}

#[get("/secrets?<guessed>")]
pub async fn get_all_secrets_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
    guessed: Option<bool>
) -> Result<Json<AllSecretsResponse>, status::Custom<Json<Value>>> {
    let with_guessed = match guessed {
        Some(value) => value,
        None => false 
    };

    let storage_secrets = storage.get_all_secrets(with_guessed).await;

    let secret_entities = match storage_secrets {
        Ok(secrets) => secrets,
        Err(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                Json(json!({"error": "Internal Server Error."})),
            ))
        }
    };

    let secrets: Vec<Secret> = secret_entities
        .into_iter()
        .map(|secret_entity| process_secret(*attempt_rule.inner(), secret_entity))
        .collect();

    let response = AllSecretsResponse {
        secrets
    };

    debug!(
        "Get All Secrets Handler executed successfully and returned {} items.",
        response.secrets.len()
    );

    Ok(Json(response))
}

#[post("/secrets/<id>", format = "json", data = "<guess>")]
pub async fn guess_secret_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
    id: &str,
    guess: Json<GuessSecretRequest>,
) -> Result<Json<Secret>, status::Custom<Json<Value>>> {
    let secret_id = match Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => {
            return Err(status::Custom(
                Status::BadRequest,
                Json(json!({"error": "Invalid Secret ID."})),
            ))
        }
    };

    let guess_data = GuessSecretRequest {
        guess: guess.guess.to_lowercase(),
        username: guess.username.clone(),
    };

    let guess_secret = storage
        .guess_secret(
            secret_id,
            guess_data.guess.clone(),
            guess_data.username.clone(),
        )
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
        Err(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                Json(json!({"error": "Internal Server Error"})),
            ))
        }
    };

    let secret = storage.get_secret_entity(secret_id).await.unwrap();

    let processed_secret = process_secret(*attempt_rule.inner(), secret);

    debug!("Guess Secret Handler executed successfully.");

    Ok(Json(processed_secret))
}
