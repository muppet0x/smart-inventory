use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use std::env;

async fn create_inventory_item(new_item: web::Json<InventoryItem>) -> impl Responder {
    HttpResponse::Ok().json({ "message": "Item added successfully", "item": new_item })
}

async fn list_all_inventory_items() -> impl Responder {
    HttpResponse::Ok().json({ "message": "Inventory List", "items": vec!["Item1", "Item2"] })
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Inventory  Item {
    name: String,
    quantity: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 

    let service_endpoint = env::var("SERVER_URL").unwrap_or_else(|_| String::from("127.0.0.1:8080"));

    println!("Starting server at {}", service_endpoint);

    HttpServer::new(|| {
        App::new()
            .route("/inventory", web::post().to(create_inventory_item))
            .route("/inventory", web::get().to(list_all_inventory_items))
    })
    .bind(&service_endpoint)?
    .run()
    .await
}