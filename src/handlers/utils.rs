use rocket::{get, post};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String
}

#[get("/health")]
pub fn health_handler() -> Json<HealthResponse> {
    let status = String::from("ok");
    let health_response = HealthResponse {status};

    Json(health_response)
}

#[derive(Deserialize)]
pub struct EchoRequest {
    text: String
}

#[derive(Serialize)]
pub struct EchoResponse {
    response: String
}

#[post("/echo", format="json", data="<text>")]
pub fn echo_handler(text: Json<EchoRequest>) -> Json<EchoResponse> {
    let response = String::from(text.text.clone());
    let response = EchoResponse {response};
    
    Json(response)
}