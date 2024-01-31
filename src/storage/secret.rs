use rocket::serde::{Deserialize, Serialize};
use mongodb::{bson::doc, results::InsertOneResult};
use uuid::Uuid;

use crate::error::AppError;

use super::storage::Storage;

#[derive(Clone, Debug, Deserialize)]
pub struct NewSecret {
    pub clue1: String,
    pub clue2: String,
    pub clue3: String,
    pub secret: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Secret {
    pub clue1: String,
    pub clue2: String,
    pub clue3: String,
    pub guessed: bool,
    pub guesser: Option<String>,
    pub id: Uuid,
    pub secret: Option<String>,
}

impl Storage {
    pub async fn create_secret(&self, new_secret: NewSecret) -> Result<InsertOneResult, AppError> {
        let secret = Secret {
            clue1: new_secret.clue1,
            clue2: new_secret.clue2,
            clue3: new_secret.clue3,
            guessed: false,
            guesser: Some("none".to_string()),
            id: Uuid::new_v4(),
            secret: Some(new_secret.secret),
        };

        Ok(self.secret_collection.insert_one(secret.clone(), None).await?)
    }

    pub async fn get_secret(&self, secret_id: Uuid) -> Result<Secret, AppError> {
        let filter = doc! {"_id": self.uuid_to_binary(secret_id) };
        match self.secret_collection.find_one(filter, None).await? {
            Some(secret) => Ok(secret),
            None => Err(AppError::NotFound)
        }
    }
}