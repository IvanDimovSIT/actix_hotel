use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, Responder};
use serde::Serialize;

pub mod hello_world;
pub mod login;
pub mod register_user;

fn serialize_output<T>(body: &T, status: StatusCode) -> HttpResponse<BoxBody>
where
    T: Serialize,
{
    let result = serde_json::to_string(body);
    if let Err(err) = result {
        return HttpResponse::InternalServerError()
            .body(format!("Error serializing output: {}", err));
    }

    HttpResponse::with_body(status, BoxBody::new(result.unwrap()))
}
