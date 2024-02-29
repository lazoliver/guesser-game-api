use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use uuid::Uuid;

use crate::storage::{
    secret::{NewSecret, Secret},
    storage::AttemptCountRule,
};
use crate::Storage;

#[derive(Serialize)]
pub struct SecretResponse {
    msg: String,
}

#[derive(Serialize)]
pub struct AllSecretsResponse {
    secrets: Vec<Secret>,
}

#[post("/secret", format = "json", data = "<secret>")]
pub async fn create_secret_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
    secret: Json<NewSecret>,
) -> Json<SecretResponse> {
    let secret = storage
        .create_secret(*attempt_rule.inner(), secret.into_inner())
        .await
        .unwrap();

    let response = SecretResponse {
        msg: String::from("Secret successfully created."),
    };

    info!("{:?}", secret);

    Json(response)
}

#[get("/secret/<id>")]
pub async fn get_secret_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
    id: &str,
) -> Json<Secret> {
    let plain_id = Uuid::parse_str(id).unwrap();

    let secret = storage.get_secret_entity(plain_id).await.unwrap();

    let processed_secret = storage.process_secret(*attempt_rule.inner(), secret);

    Json(processed_secret)
}

#[get("/secret/all")]
pub async fn get_all_secrets_handler(
    storage: &State<Storage>,
    attempt_rule: &State<AttemptCountRule>,
) -> Json<AllSecretsResponse> {
    let secrets = storage
        .get_all_secrets(*attempt_rule.inner())
        .await
        .unwrap();

    let response = AllSecretsResponse {
        secrets: secrets.clone(),
    };

    info!("Secrets array length: {}", secrets.len());

    Json(response)
}
