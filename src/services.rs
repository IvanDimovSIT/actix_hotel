use std::error::Error;

use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use utoipa::ToSchema;

pub mod add_room;
pub mod change_password;
pub mod hello_world;
pub mod login;
pub mod promote;
pub mod refresh_token;
pub mod register_user;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ErrorReponse {
    error: String,
}

pub fn error_to_response(err: Box<dyn Error>) -> HttpResponse<BoxBody> {
    error_response(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn error_response(body: String, status: StatusCode) -> HttpResponse<BoxBody> {
    serialize_output(&ErrorReponse { error: body }, status)
}

fn serialize_output<T>(body: &T, status: StatusCode) -> HttpResponse<BoxBody>
where
    T: Serialize,
{
    let result = serde_json::to_string(body);
    if let Err(err) = result {
        return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::plaintext())
            .body(format!("Error serializing output: {}", err));
    }

    HttpResponse::build(status)
        .content_type(ContentType::json())
        .body(BoxBody::new(result.unwrap()))
}
