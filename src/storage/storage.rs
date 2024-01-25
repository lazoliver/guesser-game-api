use mongodb::{Client, options::ClientOptions, Collection, Database};

use crate::error::AppError;

use super::secret_word::SecretWord;

pub struct Storage {
    pub client: Client,
    pub db: Database,
    pub secret_word_collection: Collection<SecretWord>
}

impl Storage {
    pub async fn new(mongo_uri: String) -> Result<Self, AppError> {
        let mut client_options = ClientOptions::parse(mongo_uri.clone()).await?;
        client_options.app_name = Some("Guesser Game Api".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database("guesser-game-api");
        let secret_word_collection = db.collection::<SecretWord>("secret");

        Ok(Self {
            client,
            db,
            secret_word_collection
        })
    }
}