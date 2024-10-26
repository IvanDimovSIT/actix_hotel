use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
    Responder,
};
use utoipa::OpenApi;

use crate::{
    api::{
        login::{LoginInput, LoginOutput},
        register_user::{RegisterUserInput, RegisterUserOutput},
    },
    app_state::AppState,
    security::Claims,
    services::{login::login, register_user::register_user},
    validation::Validate,
};

#[derive(OpenApi)]
#[openapi(
    paths(register_controller, login_controller),
    components(schemas(Claims, RegisterUserInput, RegisterUserOutput, LoginInput, LoginOutput))
)]
pub struct AuthApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(register_controller);
    cfg.service(login_controller);
}

#[utoipa::path(
    responses(
        (status = 201, description = "Successful Registration", body = RegisterUserOutput),
        (status = 400, description = "Invalid input", body = String)
    ),
    request_body(
        content = RegisterUserInput,
        description = "Registration data",
        content_type = "application/json"
    )
)]
#[post("/auth/register")]
pub async fn register_controller(
    state: Data<AppState>,
    input: Json<RegisterUserInput>,
) -> impl Responder {
    let register_user_input = input.into_inner();
    if let Err(err) = register_user_input.validate(&state.validator) {
        return err;
    }

    register_user(&state.db, &register_user_input).await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successful Registration", body = LoginOutput),
        (status = 400, description = "Invalid input", body = String),
        (status = 401, description = "Invalid credentials", body = String)
    ),
    request_body(
        content = LoginInput,
        description = "Login data",
        content_type = "application/json"
    )
)]
#[post("/auth/login")]
pub async fn login_controller(state: Data<AppState>, input: Json<LoginInput>) -> impl Responder {
    let login_input = input.into_inner();
    if let Err(err) = login_input.validate(&state.validator) {
        return err;
    }

    login(&state, &login_input).await
}
