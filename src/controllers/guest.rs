use actix_web::{
    http::StatusCode,
    post,
    web::{Data, Json, ServiceConfig},
    HttpRequest, Responder,
};
use utoipa::OpenApi;

use crate::{
    api::{
        error_response::ErrorResponse,
        guest::{
            add_guest::{AddGuestInput, AddGuestOutput},
            GuestIdCard,
        },
    },
    app_state::AppState,
    persistence::user::Role,
    process_request,
    security::decode_claims,
    services::guest::add_guest::add_guest,
    validation::Validate,
};

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(schemas(ErrorResponse, GuestIdCard, AddGuestInput, AddGuestOutput))
)]
pub struct GuestApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(add_guest_controller);
}

#[utoipa::path(
    responses(
        (status = 201, description = "Successfully added guest", body = AddGuestOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid credentials", body = ErrorResponse),
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
        return err.into();
    }

    let add_guest_input = input.into_inner();
    process_request!(&state, &add_guest_input, add_guest, StatusCode::CREATED)
}
