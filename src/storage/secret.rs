use crate::error::AppError;
use crate::rocket::futures::TryStreamExt;
use log::info;
use mongodb::bson::{doc, Bson};
use rocket::tokio::process;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use uuid::Uuid;

use super::storage::Storage;
use crate::AttemptCountRule;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecretEntity {
    pub id: Uuid,
    pub secret: String,
    pub clue1: String,
    pub clue2: String,
    pub clue3: String,
    pub guess_attempts: u16,
    pub guesser: Option<String>,
    pub guessed_secret: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct NewSecret {
    pub secret: String,
    pub clue1: String,
    pub clue2: String,
    pub clue3: String,
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

impl Storage {
    pub async fn create_secret(
        &self,
        attempt_rule: AttemptCountRule,
        new_secret: NewSecret,
    ) -> Result<Secret, AppError> {
        let mut hasher = Keccak256::new();

        hasher.update(new_secret.secret);

        let hashed_secret = hex::encode(hasher.finalize().to_vec());

        let secret = SecretEntity {
            id: Uuid::new_v4(),
            secret: hashed_secret,
            clue1: new_secret.clue1,
            clue2: new_secret.clue2,
            clue3: new_secret.clue3,
            guess_attempts: 0,
            guesser: None,
            guessed_secret: None,
        };

        self.secret_collection
            .insert_one(secret.clone(), None)
            .await?;

        let created_secret = self.get_secret_entity(secret.id).await?;

        let processed_secret = self.process_secret(attempt_rule, created_secret);

        debug!("New secret created: {:?}", processed_secret.clone());

        Ok(processed_secret)
    }

    pub async fn get_secret_entity(&self, secret_id: Uuid) -> Result<SecretEntity, AppError> {
        let filter = doc! {"id": self.uuid_to_binary(secret_id)};
        match self.secret_collection.find_one(filter, None).await? {
            Some(secret_entity) => Ok(secret_entity),
            None => Err(AppError::NotFound),
        }
    }

    pub async fn get_all_secrets(
        &self,
        attempt_rule: AttemptCountRule,
    ) -> Result<Vec<Secret>, AppError> {
        let filter = doc! {"guesser": None::<String>};

        let mut cursor = self.secret_collection.find(filter, None).await?;

        let mut secrets = Vec::<Secret>::new();

        while let Some(secret) = cursor.try_next().await? {
            let processed_secret = self.process_secret(attempt_rule, secret);
            secrets.push(processed_secret)
        }

        debug!("Non solved Secrets array has {} items", secrets.len());

        return Ok(secrets);
    }

    pub async fn guess_secret(
        &self,
        secret_id: Uuid,
        guess: String,
        username: String,
    ) -> Result<SecretEntity, AppError> {
        let secret = self.get_secret_entity(secret_id).await?;

        if secret.guesser.is_some() {
            return Err(AppError::AlreadyGuessed);
        };

        let mut hasher = Keccak256::new();

        hasher.update(guess.clone());

        let hashed_guess = hex::encode(hasher.finalize().to_vec());

        if hashed_guess != secret.secret {
            let update_attempts = secret.guess_attempts + 1;

            let filter = doc! {"id": self.uuid_to_binary(secret_id)};

            let updated_attempts =
                doc! {"$set": {"guess_attempts": Bson::Int32(update_attempts as i32)}};

            self.secret_collection
                .update_one(filter, updated_attempts, None)
                .await?;

            let secret = self.get_secret_entity(secret_id.clone()).await?;

            return Ok(secret);
        };

        let filter = doc! {"id": self.uuid_to_binary(secret_id)};

        let update_secret_entity = doc! {"$set": {"guesser": username, "guessed_secret": guess}};

        self.secret_collection
            .update_one(filter, update_secret_entity, None)
            .await?;

        let secret = self.get_secret_entity(secret_id.clone()).await?;

        return Ok(secret);
    }

    pub fn process_secret(&self, attempt_rule: AttemptCountRule, entity: SecretEntity) -> Secret {
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
}
