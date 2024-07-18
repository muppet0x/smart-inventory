use std::env;
use dotenv::dotenv;

pub struct Services {
    database_url: String,
}

impl Services {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_UTILS is not set in .env file");

        Services {
            database_url,
        }
    }

    pub fn init(&self) {
        println!("Initializing services with database URL: {}", self.database_url);
    }
}

fn main() {
    let services = Services::new();

    services.init();

    println!("Services initialized successfully");
}