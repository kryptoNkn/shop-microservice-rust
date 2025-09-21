use actix_web::{get, HttpResponse};
use crate::models::{Product, ProductCatalog};

#[get("/products")]
pub async fn list_products() -> HttpResponse {
    let catalog = ProductCatalog {
        items: vec![
            Product { id: 1, name: "Apple".to_string(), price: 5.0 },
            Product { id: 2, name: "Iphone".to_string(), price: 1000.0 },
            Product { id: 3, name: "BMW M5".to_string(), price: 30000.0 },
            Product { id: 4, name: "Banana".to_string(), price: 3.0 },
        ],
    };
    HttpResponse::Ok().json(catalog)
}