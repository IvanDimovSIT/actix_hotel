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
    services::guest::add_guest::add_guest,
    util::process_request_secured,
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
    process_request_secured(
        req,
        &[Role::Admin],
        &state,
        input.into_inner(),
        add_guest,
        StatusCode::CREATED,
    )
    .await
}
