use crate::{api::error_response::ErrorResponse, services::hello_world::hello_world};
use actix_web::{
    get,
    web::{Path, ServiceConfig},
    Responder,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(hello_world_controller), components(schemas(ErrorResponse)))]
pub struct HelloWorldApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(hello_world_controller);
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successful Hello Response", body = String),
        (status = 400, description = "Invalid name response", body = ErrorResponse)
    ),
    params(
        ("name" = String, Path, description = "Name to greet")
    )
)]
#[get("/hello/{name}")]
pub async fn hello_world_controller(path: Path<String>) -> impl Responder {
    let name = path.into_inner();
    hello_world(&name).await
}
