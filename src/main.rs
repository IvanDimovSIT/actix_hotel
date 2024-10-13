use actix_web::{App, HttpServer};
use constants::REST_HOST;
use controllers::hello_world::{self, hello_world_controller};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};


mod api;
mod constants;
mod controllers;
mod persistence;
mod services;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on {}:{}", REST_HOST.0, REST_HOST.1);
    HttpServer::new(|| App::new()
            .service(SwaggerUi::new("/swagger-ui/{_:.*}")
                .config(Config::default())
                .url("/api-doc/hello-world-openapi.json", hello_world::HelloWorldApiDoc::openapi())
            )
            .configure(hello_world::config)
        )
        .bind(REST_HOST)?
        .run()
        .await
}

