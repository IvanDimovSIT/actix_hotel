use actix_web::{App, HttpServer};
use app_state::AppState;
use constants::REST_HOST;
use controllers::hello_world::{self};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};


mod api;
mod constants;
mod controllers;
mod persistence;
mod services;
mod app_state;
mod security;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState::load().await;

    println!("Starting server on {}:{}", REST_HOST.0, REST_HOST.1);
    HttpServer::new(move || App::new()
            .app_data(app_state.clone())
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

