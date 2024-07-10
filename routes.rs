use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::env;

struct AppState {
}

async fn login(data: web::Json<UserLogin>) -> impl Responder {
    HttpResponse::Ok().body("User logged in")
}

async fn register_user(data: web::Json<UserRegister>) -> impl Responder {
    HttpResponse::Ok().body("User registered")
}

async fn add_inventory_item(data: web::Json<InventoryItem>) -> impl Responder {
    HttpResponse::Ok().body("Inventory item added")
}

async fn delete_inventory_item(item_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body("Inventory item deleted")
}

async fn update_inventory_item(item_id: web::Path<i32>, data: web::Json<InventoryItem>) -> impl Responder {
    HttpResponse::Ok().body("Inventory item updated")
}

async fn list_inventory_items() -> impl Responder {
    HttpResponse::Ok().body("List of inventory items")
}

async fn add_supplier(data: web::Json<Supplier>) -> impl Responder {
    HttpResponse::Ok().body("Supplier added")
}

async fn list_suppliers() -> impl Responder {
    HttpResponse::Ok().body("List of suppliers")
}

async fn inventory_status() -> impl Responder {
    HttpResponse::Ok().body("Inventory status")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");

    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register_user))
            .route("/inventory/add", web::post().to(add_inventory_item))
            .route("/inventory/delete/{item_id}", web::delete().to(delete_inventory_item))
            .route("/inventory/update/{item_id}", web::put().to(update_inventory_item))
            .route("/inventory/list", web::get().to(list_inventory_items))
            .route("/supplier/add", web::post().to(add_supplier))
            .route("/supplier/list", web::get().to(list_suppliers))
            .route("/inventory/status", web::get().to(inventory_status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}