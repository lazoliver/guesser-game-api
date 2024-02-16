use crate::{error::AppError};
use rocket::tokio::process;
use serde::{Serialize, Deserialize};
use mongodb::{bson::doc, results::InsertOneResult};
use uuid::Uuid;

use sha3::{Digest, Keccak256};

use super::storage::{AttemptCountRule, Storage};

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl Storage {
    pub async fn create_secret(&self, new_secret: NewSecret) -> Result<SecretEntity, AppError> {
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

        self.secret_collection.insert_one(secret.clone(), None).await?;

        let created_secret = self.get_secret_entity(secret.id).await?;

        Ok(created_secret)
    }

    pub async fn get_secret_entity(&self, secret_id: Uuid) -> Result<SecretEntity, AppError> {
        let filter = doc! {"id": self.uuid_to_binary(secret_id)};
        match self.secret_collection.find_one(filter, None).await? {
            Some(secret_entity) => Ok(secret_entity),
            None => Err(AppError::NotFound)
        }
    }

    pub async fn guess_secret(&self, secret_id: Uuid, guess: String, username: String) {
        // 1. Pegar segredo do banco;
        // 2. Verificar guesser, se existir retornar erro, se não seguir, AppError<AlreadyGuessed>;
        // 3. Fazer hash/encode da guess;
        // 4. Checar se a hash de guess e o secret são iguais, se sim salva no banco de dados o SecretEntity com Guesser e Guessed_Secret e retonra o segredo;
        // 5. Se não forem iguas as hashs de Guess e Secret, aumentar o contador de attempts e retorna o segredo;
    }
}
