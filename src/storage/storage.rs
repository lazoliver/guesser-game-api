use mongodb::{Client, options::ClientOptions, Collection, Database, bson::doc};

use crate::error::AppError;

use super::secret::Secret;

pub struct Storage {
    pub client: Client,
    pub db: Database,
    pub secret_collection: Collection<Secret>
}

impl Storage {
    pub async fn new(mongo_uri: String) -> Result<Self, AppError> {
        let mut client_options = ClientOptions::parse(mongo_uri.clone()).await?;
        client_options.app_name = Some("Guesser Game Api".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database("guesser-game-api");
        let secret_collection = db.collection::<Secret>("secret");

        Ok(Self {
            client,
            db,
            secret_collection
        })
    }

    pub async fn health_check(&self) -> bool {
        match self.db.run_command(doc!{"ismaster": 1}, None).await {
            Ok(_document) => return true,
            Err(e) => {
                error!("Error getting MongoDB health status: {}", e.to_string());
                return false
            }
        }
    }
}