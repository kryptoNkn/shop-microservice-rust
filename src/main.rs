mod models;
mod handlers {
    pub mod auth;
    pub mod cart;
}

use actix_web::{App, HttpServer};
use handlers::auth::{login, check, logout};
use handlers::cart::{add_to_cart, remove_from_cart, view_cart};
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(login)
            .service(check)
            .service(logout)
            .service(add_to_cart)
            .service(remove_from_cart)
            .service(view_cart)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
