use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProductCatalog {
    pub items: Vec<Product>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Cart {
    pub items: Vec<Product>,
}

impl Cart {
    pub fn add_item(&mut self, item: Product) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, id: u32) {
        self.items.retain(|p| p.id != id);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub msg: String,
}
