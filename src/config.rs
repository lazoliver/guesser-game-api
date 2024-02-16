use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseMode {
    Dev,
    Prod
}

fn default_api_port() -> u16 {
    4000
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_api_port")]
    pub api_port: u16,
    pub mongo_uri: String,
    pub release_mode: ReleaseMode
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        let config = envy::from_env::<Config>().expect("Error processing config object");

        config
    }
}