use std::env;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Supplier {
    id: String,
    name: String,
    contact_details: String,
}

struct SupplierService {
    suppliers: HashMap<String, Supplier>,
}

impl SupplierService {
    fn new() -> Self {
        SupplierService {
            suppliers: HashMap::new(),
        }
    }

    fn add_supplier(&mut self, id: String, name: String, contact_details: String) -> bool {
        if self.suppliers.contains_key(&id) {
            false
        } else {
            self.suppliers.insert(id.clone(), Supplier { id, name, contact_details });
            true
        }
    }

    fn update_supplier(&mut self, id: &str, name: Option<String>, contact_details: Option<String>) -> bool {
        self.suppliers.get_mut(id).map(|supplier| {
            if let Some(name) = name {
                supplier.name = name;
            }
            if let Some(contact_details) = contact_details {
                supplier.contact_details = contact_details;
            }
        }).is_some()
    }

    fn delete_supplier(&mut self, id: &str) -> bool {
        self.suppliers.remove(id).is_some()
    }

    fn get_supplier(&self, id: &str) -> Option<&Supplier> {
        self.suppliers.get(id)
    }
}

fn main() {
    let mut supplier_service = SupplierService::new();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Using database at: {}", db_url);

    supplier_service.add_supplier("1".to_string(), "Supplier 1".to_string(), "Contact 1".to_string());
    let maybe_supplier = supplier_service.get_supplier("1");
    println!("{:?}", maybe_supplier);
}