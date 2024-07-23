use std::collections::HashMap;
use std::env;

#[derive(Clone, Debug)]
struct InventoryItem {
    id: u32,
    name: String,
    quantity: u32,
}

struct InventoryService {
    items: HashMap<u32, InventoryItem>,
}

impl InventoryService {
    fn new() -> InventoryService {
        InventoryService {
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, id: u32, name: String, quantity: u32) {
        let item = InventoryItem { id, name, quantity };
        self.items.insert(id, item);
    }

    fn update_item(&mut self, id: u32, new_name: Option<String>, new_quantity: Option<u32>) {
        if let Some(item) = self.items.get_mut(&id) {
            if let Some(name) = new_name {
                item.name = name;
            }
            if let Some(quantity) = new_quantity {
                item.quantity = quantity;
            }
        }
    }

    fn delete_item(&mut self, id: u32) -> bool {
        self.items.remove(&id).is_some()
    }

    fn get_inventory_level(&self, id: u32) -> Option<u32> {
        self.items.get(&id).map(|item| item.quantity)
    }

    fn list_inventory(&self) -> Vec<InventoryItem> {
        self.items.values().cloned().collect()
    }
}

fn main() {
    dotenv::dotenv().ok();
    let mut inventory_service = InventoryService::new();

    inventory_service.add_item(1, "Item 1".to_string(), 10);
    inventory_service.update_item(1, Some("Updated Item 1".to_string()), Some(15));
    let deleted = inventory_service.delete_item(1);
    println!("Item deleted: {}", deleted);
    let stock_level = inventory service.get_inventory_level(1);
    println!("Stock level: {:?}", stock_level);
}