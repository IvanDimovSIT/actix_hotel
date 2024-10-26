use actix_web::{middleware::Logger, web, App, HttpServer};
use app_state::AppState;
use constants::REST_HOST;
use controllers::{
    auth,
    hello_world::{self},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

mod api;
mod app_state;
mod constants;
mod controllers;
mod persistence;
mod security;
mod services;
mod validation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState::load().await;
    println!("App state initialised");
    println!("Starting server on {}:{}", REST_HOST.0, REST_HOST.1);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .config(Config::default())
                    .url("/api-doc/openapi.json", controllers::ApiDoc::new()),
            )
            .configure(hello_world::config)
            .configure(auth::config)
    })
    .bind(REST_HOST)?
    .run()
    .await
}
