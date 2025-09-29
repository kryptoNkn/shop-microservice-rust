mod models;
mod handlers {
    pub mod auth;
    pub mod cart;
    pub mod products;
}
mod routes;

use actix_web::{App, HttpServer};
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new().configure(routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
