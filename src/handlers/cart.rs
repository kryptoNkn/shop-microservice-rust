use actix_web::{get, post, HttpRequest, HttpResponse, web, cookie::Cookie};
use crate::models::{Cart, Product};
use base64::{encode, decode};
use std::str;

fn read_cart_cookie(req: &HttpRequest) -> Cart {
    if let Some(cookie) = req.cookie("cart") {
        let decoded = decode(cookie.value()).unwrap_or_default();
        if let Ok(json_str) = str::from_utf8(&decoded) {
            if let Ok(cart) = serde_json::from_str::<Cart>(json_str) {
                return cart;
            }
        }
    }
    Cart::default()
}

fn write_cart_cookie(cart: &Cart) -> Cookie<'static> {
    let json = serde_json::to_string(cart).unwrap();
    let encoded = encode(json);
    Cookie::build("cart", encoded)
        .http_only(true)
        .finish()
}

#[post("/cart/add")]
pub async fn add_to_cart(req: HttpRequest, product: web::Json<Product>) -> HttpResponse {
    let mut cart = read_cart_cookie(&req);
    cart.items.push(product.into_inner());
    let cookie = write_cart_cookie(&cart);

    HttpResponse::Ok()
        .cookie(cookie)
        .json(&cart)
}

#[post("/cart/remove")]
pub async fn remove_from_cart(req: HttpRequest, product: web::Json<Product>) -> HttpResponse {
    let mut cart = read_cart_cookie(&req);
    cart.items.retain(|p| p.id != product.id);
    let cookie = write_cart_cookie(&cart);

    HttpResponse::Ok()
        .cookie(cookie)
        .json(&cart)
}

#[get("/cart")]
pub async fn view_cart(req: HttpRequest) -> HttpResponse {
    let cart = read_cart_cookie(&req);
    HttpResponse::Ok().json(&cart)
}
