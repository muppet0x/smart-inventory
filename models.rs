use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::sync::MutexGuard;

#[derive(Debug, Error)]
pub enum InventoryError {
    #[error("database connection failed")]
    DatabaseConnectionError,
    #[error("failed to insert into database")]
    DatabaseInsertionError,
    #[error("internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    name: String,
    price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Supplier {
    id: i32,
    name: String,
    contact_email: String,
    phone_number: String,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("failed to send request")]
    RequestFailed,
    #[error("failed to parse response")]
    ResponseParsingFailed,
    #[error(transparent)]
    InventoryError(#[from] InventoryError),
}

static SUPPLIER_CACHE: Lazy<Mutex<HashMap<i32, Supplier>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn fetch_supplier_details(supplier_id: i32) -> Result<Supplier, ApiError> {
    let cache = SUPPLIER_CACHE.lock().map_err(|e| InventoryError::InternalError(format!("Failed to lock SUPPLIER_CACHE: {}", e)))?;
    
    if let Some(supplier) = cache.get(&supplier_id) {
        println!("Cache Hit for supplier_id: {}", supplier_id);
        return Ok(supplier.clone());
    }
    drop(cache); 
    
    let api_result: Option<Supplier> = Some(Supplier {
        id: supplier_id,
        name: String::from("Test Supplier"),
        contact_email: String::from("test@example.com"),
        phone_number: String::from("123-456-7890"),
    });

    match api_result {
        Some(supplier) => {
            let mut cache = SUPPLIER_CACHE.lock().map_err(|e| InventoryError::InternalError(format!("Failed to lock SUPPLIER_CACHE for writing: {}", e)))?;
            cache.insert(supplier_id, supplier.clone());
            Ok(supplier)
        },
        None => Err(ApiError::ResponseParsingFailed),
    }
}

fn add_item_to_database(item: Item) -> Result<(), InventoryError> {
    let result = true; 

    if result {
        Ok(())
    } else {
        Err(InventoryError::DatabaseInsertionError)
    }
}

fn validate_item(item: &Item) -> Result<(), &'static str> {
    if item.name.trim().is_empty() {
        return Err("Item name cannot be empty");
    }

    if item.price <= 0.0 {
        return Err("Item price must be greater than 0");
    }

    Ok(())
}