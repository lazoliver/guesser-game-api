use rocket::serde::{uuid::Uuid, Deserialize, Serialize};
use mongodb::results::InsertOneResult;
use crate::error::AppError;

use super::storage::Storage;

#[derive(Clone, Debug, Deserialize)]
pub struct NewSecret {
    pub clue1: String,
    pub clue2: String,
    pub clue3: String,
    pub secret: String
}

#[derive(Clone, Serialize)]
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
}