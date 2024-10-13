use actix_web::{get, web, Responder};
use utoipa::OpenApi;
use crate::services::hello_world::hello_world;

#[derive(OpenApi)]
#[openapi(paths(hello_world_controller))]
pub struct HelloWorldApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world_controller);
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successful Hello Response", body = String),
        (status = 400, description = "Invalid name response", body = String)
    ),
    params(
        ("name" = String, Path, description = "Name to greet")
    )
)]
#[get("/hello/{name}")]
pub async fn hello_world_controller(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    hello_world(&name).await
}
