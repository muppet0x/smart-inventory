use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn new() -> Result<Self, envy::Error> {
        dotenv::dotenv().ok();

        envy::from_env::<Config>()
    }
}

fn main() -> Result<(), envy::Error> {
    let config = Config::new()?;

    println!("Database URL: {}", config.database_url);
    println!("Server Port: {}", config.server_port);

    Ok(())
}