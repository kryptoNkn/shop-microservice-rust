use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Cart {
    pub items: Vec<Product>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub msg: String,
}
