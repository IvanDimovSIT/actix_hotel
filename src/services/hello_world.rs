use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, Responder,
};

use crate::{api::error_response::ErrorResponse, util::serialize_to_http_response};

pub async fn hello_world(name: &str) -> impl Responder {
    if name.to_lowercase() == "error" {
        let err = ErrorResponse::new(format!("Invalid name: {}", name), StatusCode::BAD_REQUEST);
        serialize_to_http_response(&err, StatusCode::BAD_REQUEST)
    } else {
        HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::plaintext())
            .body(format!("Hello, {}!", name))
    }
}
