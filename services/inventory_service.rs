use std::collections::HashMap;

#[derive(Debug, Clone)]
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

    fn add_item(&mut self, item: InventoryItem) {
        self.items.insert(item.id, item);
    }

    fn update_item(&mut self, id: u32, new_name: Option<String>, new_quantity: Option<u32>) {
        if let Some(item) = self.items.get_mut(&id) {
            self.update_item_name(item, new_name);
            self.update_item_quantity(item, new_quantity);
        }
    }

    fn update_item_name(item: &mut InventoryItem, new_name: Option<String>) {
        if let Some(name) = new_name {
            item.name = name;
        }
    }

    fn update_item_quantity(item: &mut InventoryItem, new_quantity: Option<u32>) {
        if let Some(quantity) = new_quantity {
            item.quantity = quantity;
        }
    }

    fn delete_item(&mut self, id: u32) -> bool {
        self.items.remove(&id).is_some()
    }

    fn get_inventory_level(&self, id: u32) -> Option<u32> {
        self.items.get(&id).map(|item| item.quantity)
    }

    fn list_inventory(&self) -> Vec<&InventoryItem> {
        self.items.values().collect()
    }
}

fn main() {
    let mut inventory_service = InventoryService::new();

    let item_1 = InventoryItem { id: 1, name: "Item 1".to_string(), quantity: 10 };
    inventory_service.add_item(item_1);

    inventory_service.update_item(1, Some("Updated Item 1".to_string()), Some(15));

    let deleted = inventory_service.delete_item(1);
    println!("Item deleted: {}", deleted);

    let stock_level = inventory_service.get_inventory_level(1);
    println!("Stock level: {:?}", stock_level);
}