use actix_web::{middleware::Logger, web, App, HttpServer};
use app_state::AppState;
use constants::{APP_DEFAULT_LOGGING_LEVEL, REST_HOST};
use controllers::{auth, booking, comment, guest, room};
use cronjobs::start_cronjobs;
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

mod api;
mod app_state;
mod constants;
mod controllers;
mod cronjobs;
mod persistence;
mod security;
mod services;
mod util;
mod validation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(APP_DEFAULT_LOGGING_LEVEL));
    let app_state: AppState = AppState::load().await;
    start_cronjobs(app_state.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .config(Config::default())
                    .url("/api-doc/openapi.json", controllers::ApiDoc::new()),
            )
            .configure(auth::config)
            .configure(room::config)
            .configure(guest::config)
            .configure(booking::config)
            .configure(comment::config)
    })
    .bind(REST_HOST)?
    .run()
    .await
}
