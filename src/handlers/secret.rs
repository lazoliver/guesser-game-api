use rocket::{get, post};
use rocket::State;
use crate::error::AppError;
use crate::{Storage, storage::secret::NewSecret};
use rocket::serde::{Serialize, Deserialize, Serializer};
use rocket::serde::json::Json;

#[derive(Deserialize)]
pub struct SecretRequest {
    new_secret: NewSecret
}

#[derive(Serialize)]
pub struct SecretResponse {
    status: String
}

#[post("/secret", format="json", data="<secret>")]
pub async fn create_secret_handler(storage: &State<Storage>, secret: Json<SecretRequest>) -> Json<SecretResponse> {
    let secret = NewSecret {
        clue1: secret.new_secret.clue1.to_string(),
        clue2: secret.new_secret.clue2.to_string(),
        clue3: secret.new_secret.clue3.to_string(),
        secret: secret.new_secret.secret.clone()
    };

    storage.create_secret(secret).await.unwrap();

    let response = String::from("OK");
    let secret_response = SecretResponse {status: response};

    Json(secret_response)
}