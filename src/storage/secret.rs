use crate::{error::AppError};
use rocket::tokio::process;
use serde::{Serialize, Deserialize};
use mongodb::{bson::doc, results::InsertOneResult};
use uuid::Uuid;

use super::storage::{AttemptCountRule, Storage};

#[derive(Clone, Deserialize, Serialize)]
pub struct SecretEntity {
    pub id: Uuid,
    pub secret: String,
    pub clue1: String,
    pub clue2: String,
    pub clue3: String,
    pub guess_attempts: u16,
    pub guesser: Option<String>,
    pub guessed_secret: Option<String>
}

pub struct NewSecret {
    pub secret: String,
    pub clue1: String,
    pub clue2: String,
    pub clue3: String
}

pub struct Secret {
    pub id: Uuid,
    pub guessed: bool,
    pub clue1: Option<String>,
    pub clue2: Option<String>,
    pub clue3: Option<String>,
    pub guesser: Option<String>,
    pub secret: Option<String>
}

impl Storage {
    pub async fn create_secret(&self, new_secret: NewSecret, attempt_rule: AttemptCountRule) -> Result<Secret, AppError> {
        let secret = SecretEntity {
            id: Uuid::new_v4(),
            secret: new_secret.secret,
            clue1: new_secret.clue1,
            clue2: new_secret.clue2,
            clue3: new_secret.clue3,
            guess_attempts: 0,
            guesser: None,
            guessed_secret: None,
        };

        self.secret_collection.insert_one(secret.clone(), None).await?;

        let created_secret = self.get_secret_entity(secret.id).await?;

        Ok(self.process_secret(attempt_rule, created_secret))
    }

    pub async fn get_secret_entity(&self, secret_id: Uuid) -> Result<SecretEntity, AppError> {
        let filter = doc! {"id": self.uuid_to_binary(secret_id)};
        match self.secret_collection.find_one(filter, None).await? {
            Some(secret_entity) => Ok(secret_entity),
            None => Err(AppError::NotFound)
        }
    }

    pub fn process_secret(&self, attempt_rule: AttemptCountRule, entity: SecretEntity) -> Secret {
        Secret {
            id: entity.id,
            guessed: entity.guesser.is_some(),
            clue1: match (entity.guess_attempts >= attempt_rule.clue1_attempts) {
                true => Some(entity.clue1),
                false => None
            },
            clue2: match (entity.guess_attempts >= attempt_rule.clue2_attempts) {
                true => Some(entity.clue2),
                false => None
            },
            clue3: match (entity.guess_attempts >= attempt_rule.clue3_attempts) {
                true => Some(entity.clue3),
                false => None
            },
            secret: entity.guessed_secret,
            guesser: entity.guesser
        }
    }
}
