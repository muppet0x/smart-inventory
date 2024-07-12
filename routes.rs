use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::env;

struct AppState {
}

async fn login_user(data: web::Json<UserLoginRequest>) -> impl Responder {
    HttpResponse::Ok().body("User logged in")
}

async fn register_new_user(data: web::Json<UserRegistrationRequest>) -> impl Responder {
    HttpResponse::Ok().body("User registered")
}

async fn add_new_inventory_item(data: web::Json<NewInventoryItemRequest>) -> impl Responder {
    HttpResponse::Ok().body("Inventory item added")
}

async fn remove_inventory_item(item_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body("Inventory item deleted")
}

async fn modify_inventory_item(item_id: web::Path<i32>, data: web::Json<UpdateInventoryItemRequest>) -> impl Responder {
    HttpResponse::Ok().body("Inventory item updated")
}

async fn fetch_inventory_items() -> impl Responder {
    HttpResponse::Ok().body("List of inventory items")
}

async fn register_new_supplier(data: web::Json<NewSupplierRequest>) -> impl Responder {
    HttpResponse::Ok().body("Supplier added")
}

async fn retrieve_suppliers() -> impl Responder {
    HttpResponse::Ok().body("List of suppliers")
}

async fn check_inventory_status() -> impl Responder {
    HttpResponse::Ok().body("Inventory status")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(login_user))
            .route("/register", web::post().to(register_new_user))
            .route("/inventory/add", web::post().to(add_new_inventory_item))
            .route("/inventory/delete/{item_id}", web::delete().to(remove_inventory_item))
            .route("/inventory/update/{item_id}", web::put().to(modify_inventory_item))
            .route("/inventory/list", web::get().to(fetch_inventory_items))
            .route("/supplier/add", web::post().to(register_new_supplier))
            .route("/supplier/list", web::get().to(retrieve_suppliers))
            .route("/inventory/status", web::get().to(check_inventory_status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}