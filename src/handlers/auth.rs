use actix_web::{post, get, HttpResponse, HttpRequest, cookie::Cookie};
use crate::models::Message;

#[post("/login")]
pub async fn login() -> HttpResponse {
    let token = "user_token_123";

    let cookie = Cookie::build("auth_token", token)
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(Message { msg: "Logged in".to_string() })
}

#[get("/check")]
pub async fn check(req: HttpRequest) -> HttpResponse {
    if let Some(cookie) = req.cookie("auth_token") {
        HttpResponse::Ok().json(Message { msg: format!("Token: {}", cookie.value()) })
    } else {
        HttpResponse::Unauthorized().json(Message { msg: "Not authorized".to_string() })
    }
}
