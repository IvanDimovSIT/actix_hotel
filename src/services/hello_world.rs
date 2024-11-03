use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, Responder,
};

use super::error_response;

pub async fn hello_world(name: &str) -> impl Responder {
    if name.to_lowercase() == "error" {
        error_response(format!("Invalid name: {}", name), StatusCode::BAD_REQUEST)
    } else {
        HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::plaintext())
            .body(format!("Hello, {}!", name))
    }
}
