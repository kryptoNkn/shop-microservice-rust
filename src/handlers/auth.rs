use actix_web::{post, get, HttpResponse, HttpRequest, cookie::{Cookie, SameSite}};
use crate::models::Message;
use log::warn;
use actix_web::cookie::time::Duration;


#[post("/login")]
pub async fn login() -> HttpResponse {
    let token = "user_token_123";

    let cookie = Cookie::build("auth_token", token)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(Message { msg: "Logged in".to_string() })
}

#[get("/check")]
pub async fn check(req: HttpRequest) -> HttpResponse {
    match req.cookie("auth_token") {
        Some(cookie) => HttpResponse::Ok()
            .json(Message { msg: format!("Token: {}", cookie.value()) }),
        None => {
            warn!("Unauthorized access attempt");
            HttpResponse::Unauthorized()
                .json(Message { msg: "Not authorized".to_string() })
        }
    }
}

#[post("/logout")]
pub async fn logout() -> HttpResponse {
    let cookie = Cookie::build("auth_token", "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::seconds(0))
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(Message { msg: "Logged out".to_string() })
}
