use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InventoryError {
    #[error("database connection failed")]
    DatabaseConnectionError,
    #[error("failed to insert into database")]
    DatabaseInsertionError,
    // Add more specific errors as needed
}

fn add_item_to_database(item: Item) -> Result<(), InventoryError> {
    // Placeholder for actual database code
    let result = true; // Simulating database operation success

    if result {
        Ok(())
    } else {
        Err(InventoryError::DatabaseInsertionError)
    }
}
```
```rust
fn validate_item(item: &Item) -> Result<(), &'static str> {
    if item.name.trim().is_empty() {
        return Err("Item name cannot be empty");
    }

    if item.price <= 0.0 {
        return Err("Item price must be greater than 0");
    }

    // Add more validations as needed

    Ok(())
}
```
```rust
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("failed to send request")]
    RequestFailed,
    #[error("failed to parse response")]
    ResponseParsingFailed,
    // More specific errors can be added here
}

fn fetch_supplier_details(supplier_id: i32) -> Result<Supplier, ApiInfoError> {
    // Placeholder for actual API call code
    let result: Option<Supplier> = Some(Supplier {
        id: supplier_id,
        name: String::from("Test Supplier"),
        contact_email: String::from("test@example.com"),
        phone_number: String::from("123-456-7890"),
    });

    match result {
        Some(supplier) => Ok(supplier),
        None => Err(ApiError::ResponseParsingFailed),
    }
}