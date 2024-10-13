use actix_web::{HttpResponse, Responder};

pub async fn hello_world(name: &str) -> impl Responder {
    if name.to_lowercase() == "error" {
        HttpResponse::BadRequest().body(format!("Invalid name: {}", name))
    } else {
        HttpResponse::Ok().body(format!("Hello, {}!", name))
    }
}