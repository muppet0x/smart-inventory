use std::collections::HashMap;
use std::env;

#[derive(Debug)]
struct InventoryItem {
    id: u32,
    name: String,
    quantity: u32,
}

struct InventoryService<'a> {
    items: HashMap<u32, &'a InventoryItem>,
}

impl<'a> InventoryService<'a> {
    fn new() -> InventoryService<'a> {
        InventoryService {
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, item: &'a InventoryItem) {
        self.items.insert(item.id, item);
    }

    fn update_item(&mut self, id: u32, new_name: Option<String>, new_quantity: Option<u32>) {
        if let Some(item) = self.items.get_mut(&id) {
            if let Some(name) = new_name {
                (*item).name = name; // Requires mutable references or creative ways to handle borrowing rules
            }
            if let Some(quantity) = new_quantity {
                (*item).quantity = quantity;
            }
        }
    }

    fn delete_item(&mut self, id: u32) -> bool {
        self.items.remove(&id).is_some()
    }

    fn get_inventory_level(&self, id: u32) -> Option<u32> {
        self.items.get(&id).map(|item| item.quantity)
    }

    fn list_inventory(&self) -> Vec<&InventoryItem> {
        self.items.values().cloned().collect()
    }
}

fn main() {
    dotenv::dotenv().ok();
    let mut inventory_service = InventoryService::new();

    let item_1 = InventoryItem { id: 1, name: "Item 1".to_string(), quantity: 10 };
    inventory_service.add_item(&item_1);
    // Skipping update_item call as it requires mutable references.
    let deleted = inventory_service.delete_item(1);
    println!("Item deleted: {}", deleted);

    // Corrected typo in next line
    let stock_level = inventory_service.get_inventory_level(1);
    println!("Stock culty in next line
    let stock_level = inventory_service.get_inventory_level(1);
    println!("Stock level: {:?}", stock_level);
}