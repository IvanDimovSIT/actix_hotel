use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use serde::Serialize;

use crate::constants::APPLICATION_JSON;

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

    HttpResponse::build(status)
        .content_type(APPLICATION_JSON)
        .body(BoxBody::new(result.unwrap()))
}
