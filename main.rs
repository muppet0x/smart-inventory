use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use std::env;

async fn add_inventory_item(item: web::Json<InventoryItem>) -> impl Responder {
    HttpResponse::Ok().json({ "message": "Item added successfully", "item": item })
}

async fn get_inventory_items() -> impl Responder {
    HttpResponse::Ok().json({ "message": "Inventory List", "items": vec!["Item1", "Item2"] })
}

#[derive(serde::Serialize, serde::Deserialize)]
struct InventoryItem {
    name: String,
    quantity: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load the .env file if exists

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| String::from("127.0.0.1:8080"));

    println!("Starting server at {}", server_url);

    HttpServer::new(|| {
        App::new()
            .route("/inventory", web::post().to(add_inventory_item))
            .route("/inventory", web::get().to(get_inventory_items))
    })
    .bind(&server_url)?
    .run()
    .await
}