use rocket::{get, post};
use rocket::State;
use crate::Storage;
use std::time::SystemTime;
use rocket::serde::{Serialize, Deserialize, Serializer};
use rocket::serde::json::Json;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String
}

#[derive(Deserialize)]
pub struct EchoRequest {
    text: String
}

#[derive(Serialize)]
pub struct EchoResponse {
    response: String
}

#[derive(Serialize, Debug)]
pub struct FullHealthResponse {
    status: String,
    #[serde(serialize_with = "serialize_with_two_decimals")]
    uptime: f64,
    db: bool
}

fn serialize_with_two_decimals<S>(x: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let factor = 100.0; // Change this for different decimal places
    let rounded = (x * factor).round() / factor;
    s.serialize_f64(rounded)
}

#[get("/health")]
pub fn health_handler() -> Json<HealthResponse> {
    let status = String::from("ok");
    let health_response = HealthResponse {status};

    debug!("Health handler executed successfully");

    Json(health_response)
}

#[post("/echo", format="json", data="<text>")]
pub fn echo_handler(text: Json<EchoRequest>) -> Json<EchoResponse> {
    let response = String::from(text.text.clone());
    let response = EchoResponse {response};

    debug!("Echo handler executed successfully");
    
    Json(response)
}

#[get("/health/full")]
pub async fn full_health_handler(storage: &State<Storage>, start_time: &State<SystemTime>) -> Json<FullHealthResponse> {
    let db_status = storage.health_check().await;

    let status = match db_status {
        true => "pass",
        false => "fail"
    }.to_string();

    let uptime = SystemTime::now().duration_since(*start_time.inner())
        .map(|duration| duration.as_secs_f64())
        .unwrap_or_default();

    let response = FullHealthResponse {
        status,
        uptime,
        db: db_status
    };

    Json(response)
}