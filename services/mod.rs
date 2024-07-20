use std::env;
use dotenv::dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;

pub struct Services {
    database_url: String,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Services {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");

        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        let pool = r2d2::Pool::builder().build(manager)
            .expect("Failed to create pool.");

        Services {
            database_url,
            pool,
        }
    }

    pub fn init(&self) {
        println!("Initializing services with database URL: {}", self.database_url);
        // Example on how to use the connection from pool in a real application scenario
        let _conn = self.pool.get().expect("Failed to get a connection from the pool");
        println!("Database connection established successfully.");
    }
}

fn main() {
    let services = Services::new();

    services.init();

    println!("Services initialized successfully with enhanced database connection management.");
}