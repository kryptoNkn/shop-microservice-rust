use actix_web::web;
use crate::handlers::{
    auth::{login, check, logout},
    cart::{add_to_cart, remove_from_cart, view_cart},
    products::list_products,
}; 

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
    .service(check)
    .service(logout)
    .service(add_to_cart)
    .service(remove_from_cart)
    .service(view_cart)
    .service(list_products);
}