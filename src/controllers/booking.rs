use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Json, Query, ServiceConfig},
    HttpRequest, Responder,
};
use utoipa::OpenApi;

use crate::{
    api::{
        booking::find_unoccupied_rooms::{FindUnoccupiedRoomsInput, FindUnoccupiedRoomsOutput},
        error_response::ErrorResponse,
        guest::{
            add_guest::{AddGuestInput, AddGuestOutput},
            GuestIdCard,
        },
    },
    app_state::AppState,
    persistence::user::Role,
    services::booking::find_unoccupied_rooms::find_unoccupied_rooms,
    util::process_request_secured,
};

#[derive(OpenApi)]
#[openapi(
    paths(find_unoccupied_rooms_controller),
    components(schemas(ErrorResponse, FindUnoccupiedRoomsInput, FindUnoccupiedRoomsOutput))
)]
pub struct BookingApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(find_unoccupied_rooms_controller);
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully found free rooms", body = FindUnoccupiedRoomsOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
    ),
    params(
        ("startDate" = Date, Query, description = "Start date for booking", example = "2025-01-01"),
        ("endDate" = Date, Query, description = "End date for booking", example = "2025-01-01"),
        ("minimumCapacity" = Option<i16>, Query, description = "Minimum room capacity", example = "2", nullable),
        ("maximumCapacity" = Option<i16>, Query, description = "Maximum room capacity", example = "3", nullable),
    ),
    security(("bearer_auth" = []))
)]
#[get("/booking/unoccupied")]
pub async fn find_unoccupied_rooms_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Query<FindUnoccupiedRoomsInput>,
) -> impl Responder {
    process_request_secured(
        req,
        &[Role::User, Role::Admin],
        &state,
        input.into_inner(),
        find_unoccupied_rooms,
        StatusCode::OK,
    )
    .await
}
