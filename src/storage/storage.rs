use mongodb::{
    bson::{doc, spec::BinarySubtype, Binary},
    options::ClientOptions,
    Client, Collection, Database,
};
use uuid::Uuid;

use crate::error::AppError;

use super::secret::SecretEntity;

pub struct Storage {
    pub client: Client,
    pub db: Database,
    pub secret_collection: Collection<SecretEntity>,
}

#[derive(Copy, Clone)]
pub struct AttemptCountRule {
    pub clue1_attempts: u16,
    pub clue2_attempts: u16,
    pub clue3_attempts: u16,
}

impl Storage {
    pub async fn new(mongo_uri: String) -> Result<Self, AppError> {
        let mut client_options = ClientOptions::parse(mongo_uri.clone()).await?;
        client_options.app_name = Some("Guesser Game Api".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database("guesser-game-api");
        let secret_collection = db.collection::<SecretEntity>("secrets");

        Ok(Self {
            client,
            db,
            secret_collection,
        })
    }

    pub async fn health_check(&self) -> bool {
        match self.db.run_command(doc! {"ismaster": 1}, None).await {
            Ok(_document) => return true,
            Err(e) => {
                error!("Error getting MongoDB health status: {}", e.to_string());
                return false;
            }
        }
    }

    pub fn uuid_to_binary(&self, id: Uuid) -> Binary {
        Binary {
            subtype: BinarySubtype::Generic,
            bytes: id.as_bytes().to_vec(),
        }
    }
}
