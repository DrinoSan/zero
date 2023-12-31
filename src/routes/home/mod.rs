use actix_web;
use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn home() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("home.html"))
}
