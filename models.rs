use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub stock_quantity: i32,
    pub supplier_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Supplier {
    pub id: i32,
    pub name: String,
    pub contact_email: String,
    pub phone_number: String,
}