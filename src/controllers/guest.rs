use actix_web::{
    get,
    http::StatusCode,
    post, put,
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpRequest, Responder,
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    api::{
        error_response::ErrorResponse,
        guest::{
            add_guest::{AddGuestInput, AddGuestOutput},
            find_guest::{FindGuestInput, FindGuestOutput},
            get_guest::{GetGuestInput, GetGuestOutput},
            update_guest::{UpdateGuestInput, UpdateGuestOutput},
            GuestIdCard,
        },
    },
    app_state::AppState,
    persistence::user::Role,
    services::guest::{
        add_guest::add_guest_service, find_guest::find_guest_service, get_guest::get_guest_service,
        update_guest::update_guest_service,
    },
    util::process_request_secured,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        add_guest_controller,
        find_guest_controller,
        get_guest_controller,
        update_guest_controller
    ),
    components(schemas(
        ErrorResponse,
        GuestIdCard,
        AddGuestInput,
        AddGuestOutput,
        FindGuestInput,
        FindGuestOutput,
        GetGuestInput,
        GetGuestOutput,
        UpdateGuestInput,
        UpdateGuestOutput
    ))
)]
pub struct GuestApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(add_guest_controller);
    cfg.service(find_guest_controller);
    cfg.service(get_guest_controller);
    cfg.service(update_guest_controller);
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
        add_guest_service,
        StatusCode::CREATED,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully found guests", body = FindGuestOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid authority", body = ErrorResponse),
    ),
    params(
        ("firstName" = Option<String>, Query, description = "Person's first name to search by", example = "John", nullable),
        ("lastName" = Option<String>, Query, description = "Person's last name to search by", example = "Smith", nullable),
        ("dateOfBirth" = Option<Date>, Query, description = "Person's date of birth to search by", example = "2003-11-30", nullable),
        ("ucn" = Option<String>, Query, description = "Person's UCN to search by", example = "0987654321", nullable),
        ("phoneNumber" = Option<String>, Query, description = "Person's phone number to search by", example = "+35921114567", nullable),
    ),
    security(("bearer_auth" = []))
)]
#[get("/guest")]
pub async fn find_guest_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Query<FindGuestInput>,
) -> impl Responder {
    process_request_secured(
        req,
        &[Role::Admin],
        &state,
        input.into_inner(),
        find_guest_service,
        StatusCode::OK,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully found guests", body = GetGuestOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid authority", body = ErrorResponse),
        (status = 404, description = "Guest not found", body = ErrorResponse),
    ),
    params(
        ("guestId" = String, Path, description = "Guest id")
    ),
    security(("bearer_auth" = []))
)]
#[get("/guest/{guestId}")]
pub async fn get_guest_controller(
    req: HttpRequest,
    state: Data<AppState>,
    path: Path<Uuid>,
) -> impl Responder {
    process_request_secured(
        req,
        &[Role::Admin],
        &state,
        GetGuestInput {
            guest_id: path.into_inner(),
        },
        get_guest_service,
        StatusCode::OK,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully update guest", body = UpdateGuestOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid credentials", body = ErrorResponse),
        (status = 404, description = "Guest not found", body = ErrorResponse),
    ),
    request_body(
        content = UpdateGuestInput,
        description = "Guest data",
        content_type = "application/json"
    ),
    params(
        ("guestId" = String, Path, description = "Guest id")
    ),
    security(("bearer_auth" = []))
)]
#[put("/guest/{guestId}")]
pub async fn update_guest_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Json<UpdateGuestInput>,
    path: Path<Uuid>,
) -> impl Responder {
    process_request_secured(
        req,
        &[Role::Admin],
        &state,
        UpdateGuestInput {
            id: Some(path.into_inner()),
            ..input.into_inner()
        },
        update_guest_service,
        StatusCode::OK,
    )
    .await
}
