use actix_web::{get, post, HttpRequest, HttpResponse, web, cookie::{Cookie, SameSite}};
use crate::models::{Cart, Product};
use base64::{encode, decode};
use std::str;
use log::warn;

fn read_cart_cookie(req: &HttpRequest) -> Cart {
    if req.cookie("auth_token").is_none() {
        warn!("Attempt to access cart without auth");
        return Cart::default();
    }

    if let Some(cookie) = req.cookie("cart") {
        if let Ok(decoded) = decode(cookie.value()) {
            if let Ok(json_str) = str::from_utf8(&decoded) {
                if let Ok(cart) = serde_json::from_str::<Cart>(json_str) {
                    return cart;
                } else {
                    warn!("Failed to parse JSON cart");
                }
            } else {
                warn!("UTF-8 error when reading cart");
            }
        } else {
            warn!("Base64 decode error for cart");
        }
    }
    Cart::default()
}

fn write_cart_cookie(cart: &Cart) -> Option<Cookie<'static>> {
    match serde_json::to_string(cart) {
        Ok(json) => {
            let encoded = encode(json);
            Some(
                Cookie::build("cart", encoded)
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::Lax)
                    .path("/")
                    .finish()
            )
        }
        Err(err) => {
            warn!("Failed to serialize cart: {}", err);
            None
        }
    }
}

#[post("/cart/add")]
pub async fn add_to_cart(req: HttpRequest, product: web::Json<Product>) -> HttpResponse {
    if req.cookie("auth_token").is_none() {
        return HttpResponse::Unauthorized().json("Not authorized");
    }

    let mut cart = read_cart_cookie(&req);
    cart.add_item(product.into_inner());

    let mut response = HttpResponse::Ok();
    if let Some(cookie) = write_cart_cookie(&cart) {
        response.cookie(cookie);
    }
    response.json(&cart)
}

#[post("/cart/remove")]
pub async fn remove_from_cart(req: HttpRequest, product: web::Json<Product>) -> HttpResponse {
    if req.cookie("auth_token").is_none() {
        return HttpResponse::Unauthorized().json("Not authorized");
    }

    let mut cart = read_cart_cookie(&req);
    cart.remove_item(product.id);

    let mut response = HttpResponse::Ok();
    if let Some(cookie) = write_cart_cookie(&cart) {
        response.cookie(cookie);
    }
    response.json(&cart)
}

#[get("/cart")]
pub async fn view_cart(req: HttpRequest) -> HttpResponse {
    if req.cookie("auth_token").is_none() {
        return HttpResponse::Unauthorized().json("Not authorized");
    }

    let cart = read_cart_cookie(&req);
    HttpResponse::Ok().json(&cart)
}
