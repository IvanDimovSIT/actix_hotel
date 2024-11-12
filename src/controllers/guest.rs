use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
    HttpRequest, Responder,
};
use utoipa::OpenApi;

use crate::{
    api::{
        error_response::ErrorReponse,
        guest::add_guest::{AddGuestInput, AddGuestOutput, GuestIdCard},
    },
    app_state::AppState,
    persistence::user::Role,
    security::decode_claims,
    services::guest::add_guest::add_guest,
    validation::Validate,
};

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(schemas(ErrorReponse, GuestIdCard, AddGuestInput, AddGuestOutput))
)]
pub struct GuestApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(add_guest_controller);
}

#[utoipa::path(
    responses(
        (status = 201, description = "Successfully added guest", body = AddGuestOutput),
        (status = 400, description = "Invalid input", body = ErrorReponse),
        (status = 401, description = "Invalid credentials", body = ErrorReponse),
        (status = 403, description = "Invalid credentials", body = ErrorReponse),
    ),
    request_body(
        content = AddGuestInput,
        description = "Guest data",
        content_type = "application/json"
    ),
    security(("bearer_auth" = []))
)]
#[post("/guest")]
pub async fn add_guest_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Json<AddGuestInput>,
) -> impl Responder {
    if let Err(err) = decode_claims(&req, &state, &[Role::Admin]) {
        return err;
    }

    let add_guest_input = input.into_inner();
    if let Err(err) = add_guest_input.validate(&state.validator) {
        return err;
    }

    add_guest(&state, &add_guest_input).await
}
