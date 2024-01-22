use crate::error::AppError;
use serde::{Serialize, Deserialize};
use mongodb::results::InsertOneResult;

use super::storage::Storage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecretWord {
    pub name: String,
}

impl Storage {
    pub async fn create_secret(&self, name: String) -> Result<InsertOneResult, AppError> {
        let new_secret = SecretWord { name };

        Ok(self.secret_word_collection.insert_one(new_secret.clone(), None).await?)
    }
}
