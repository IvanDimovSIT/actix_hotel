use actix_web::{
    post, put,
    web::{Data, Json, ServiceConfig},
    HttpRequest, Responder,
};
use utoipa::OpenApi;

use crate::{
    api::{
        login::{LoginInput, LoginOutput},
        promote::{PromoteInput, PromoteOutput},
        register_user::{RegisterUserInput, RegisterUserOutput},
    },
    app_state::AppState,
    persistence::user::Role,
    security::{decode_claims, Claims},
    services::{login::login, promote::promote, register_user::register_user},
    validation::Validate,
};

#[derive(OpenApi)]
#[openapi(
    paths(register_controller, login_controller),
    components(schemas(
        Claims,
        RegisterUserInput,
        RegisterUserOutput,
        LoginInput,
        LoginOutput,
        PromoteInput,
        PromoteOutput
    ))
)]
pub struct AuthApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(register_controller);
    cfg.service(login_controller);
    cfg.service(promote_controller);
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

#[utoipa::path(
    responses(
        (status = 200, description = "Successful Promotion", body = PromoteOutput),
        (status = 400, description = "Invalid input", body = String),
        (status = 401, description = "Invalid credentials", body = String),
        (status = 403, description = "Invalid credentials", body = String),
    ),
    request_body(
        content = PromoteInput,
        description = "Promote data",
        content_type = "application/json"
    ),
    security(("bearer_auth" = []))
)]
#[put("/auth/promote")]
pub async fn promote_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Json<PromoteInput>,
) -> impl Responder {
    let authorization = decode_claims(&req, &state, &[Role::Admin]);
    if let Err(err) = authorization {
        return err;
    }

    let promote_input = input.into_inner();
    if let Err(err) = promote_input.validate(&state.validator) {
        return err;
    }

    promote(&state, &promote_input).await
}
