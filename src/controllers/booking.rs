use actix_web::{
    get, http::StatusCode, post, put, web::{Data, Json, Path, Query, ServiceConfig}, HttpRequest, Responder
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    api::{
        booking::{
            book_room::{BookRoomInput, BookRoomOutput},
            find_unoccupied_rooms::{FindUnoccupiedRoomsInput, FindUnoccupiedRoomsOutput}, pay_booking::{PayBookingInput, PayBookingOutput},
        },
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::user::Role,
    services::booking::{book_room::book_room, find_unoccupied_rooms::find_unoccupied_rooms, pay_booking::pay_booking},
    util::process_request_secured,
};

#[derive(OpenApi)]
#[openapi(
    paths(find_unoccupied_rooms_controller, book_room_controller, pay_booking_controller),
    components(schemas(
        ErrorResponse,
        FindUnoccupiedRoomsInput,
        FindUnoccupiedRoomsOutput,
        BookRoomInput,
        BookRoomOutput,
        PayBookingInput,
        PayBookingOutput
    ))
)]
pub struct BookingApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(find_unoccupied_rooms_controller);
    cfg.service(book_room_controller);
    cfg.service(pay_booking_controller);
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

#[utoipa::path(
    responses(
        (status = 201, description = "Successfully booked room", body = BookRoomOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid credentials", body = ErrorResponse),
    ),
    request_body(
        content = BookRoomInput,
        description = "Booking data",
        content_type = "application/json"
    ),
    security(("bearer_auth" = []))
)]
#[post("/booking")]
pub async fn book_room_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Json<BookRoomInput>,
) -> impl Responder {
    process_request_secured(
        req,
        &[Role::Admin],
        &state,
        input.into_inner(),
        book_room,
        StatusCode::CREATED,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully booked room", body = PayBookingOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid credentials", body = ErrorResponse),
    ),
    params(
        ("bookingId" = String, Path, description = "Booking id")
    ),
    security(("bearer_auth" = []))
)]
#[put("/booking/pay/{bookingId}")]
pub async fn pay_booking_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Path<Uuid>,
) -> impl Responder {
    process_request_secured(
        req,
        &[Role::Admin],
        &state,
        PayBookingInput{ booking_id: input.into_inner() },
        pay_booking,
        StatusCode::OK,
    )
    .await
}
