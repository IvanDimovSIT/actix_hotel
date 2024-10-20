use actix_web::{post, web, Responder};
use utoipa::OpenApi;

use crate::{
    api::register_user::{RegisterUserInput, RegisterUserOutput}, app_state::AppState,
    services::register_user::register_user, validation::Validate,
};

#[derive(OpenApi)]
#[openapi(
    paths(register_controller),
    components(schemas(RegisterUserInput, RegisterUserOutput))
)]
pub struct AuthApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register_controller);
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
    state: web::Data<AppState>,
    input: web::Json<RegisterUserInput>,
) -> impl Responder {
    let register_user_input = input.into_inner();
    if let Err(err) = register_user_input.validate(&state.validator) {
        return err;
    }

    register_user(&state.db, &register_user_input).await
}
