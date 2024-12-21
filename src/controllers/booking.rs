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
        booking::{
            book_room::{BookRoomInput, BookRoomOutput},
            cancel_booking::{CancelBookingInput, CancelBookingOutput},
            find_unoccupied_rooms::{FindUnoccupiedRoomsInput, FindUnoccupiedRoomsOutput},
            get_booking::{BookingGuest, GetBookingInput, GetBookingOutput},
            get_own_bookings::{GetOwnBookingsInput, GetOwnBookingsOutput},
            pay_booking::{PayBookingInput, PayBookingOutput},
        },
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::{booking::BookingStatus, user::Role},
    services::booking::{
        book_room::book_room, cancel_booking::cancel_booking,
        find_unoccupied_rooms::find_unoccupied_rooms, get_booking::get_booking,
        get_own_bookings::get_own_bookings, pay_booking::pay_booking,
    },
    util::process_request_secured,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        find_unoccupied_rooms_controller,
        book_room_controller,
        pay_booking_controller,
        get_booking_controller,
        get_own_bookings_controller,
        cancel_booking_controller
    ),
    components(schemas(
        ErrorResponse,
        FindUnoccupiedRoomsInput,
        FindUnoccupiedRoomsOutput,
        BookRoomInput,
        BookRoomOutput,
        PayBookingInput,
        PayBookingOutput,
        BookingStatus,
        BookingGuest,
        GetBookingInput,
        GetBookingOutput,
        GetOwnBookingsInput,
        GetOwnBookingsOutput,
        CancelBookingInput,
        CancelBookingOutput
    ))
)]
pub struct BookingApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(find_unoccupied_rooms_controller);
    cfg.service(book_room_controller);
    cfg.service(pay_booking_controller);
    cfg.service(get_booking_controller);
    cfg.service(get_own_bookings_controller);
    cfg.service(cancel_booking_controller);
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
        (status = 200, description = "Successfully payed room", body = PayBookingOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid credentials", body = ErrorResponse),
        (status = 404, description = "Booking not found", body = ErrorResponse),
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
        PayBookingInput {
            booking_id: input.into_inner(),
        },
        pay_booking,
        StatusCode::OK,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully canceled booking", body = CancelBookingOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid credentials", body = ErrorResponse),
        (status = 404, description = "Booking not found", body = ErrorResponse),
    ),
    params(
        ("bookingId" = String, Path, description = "Booking id")
    ),
    security(("bearer_auth" = []))
)]
#[put("/booking/cancel/{bookingId}")]
pub async fn cancel_booking_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Path<Uuid>,
) -> impl Responder {
    process_request_secured(
        req,
        &[Role::Admin],
        &state,
        CancelBookingInput {
            booking_id: input.into_inner(),
        },
        cancel_booking,
        StatusCode::OK,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully fetched booking", body = GetBookingOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 401, description = "No access to booking", body = ErrorResponse),
        (status = 404, description = "Booking not found", body = ErrorResponse),
    ),
    params(
        ("bookingId" = String, Path, description = "Booking id")
    ),
    security(("bearer_auth" = []))
)]
#[get("/booking/{bookingId}")]
pub async fn get_booking_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Path<Uuid>,
) -> impl Responder {
    process_request_secured(
        req,
        &[Role::User, Role::Admin],
        &state,
        GetBookingInput {
            booking_id: input.into_inner(),
            ..Default::default()
        },
        get_booking,
        StatusCode::OK,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully fetched booking", body = GetBookingOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 401, description = "No access to booking", body = ErrorResponse),
        (status = 404, description = "Booking not found", body = ErrorResponse),
    ),
    params(
        ("includeCanceled" = bool, Query, description = "Should include canceld bookings", example = "true"),
        ("includePaid" = bool, Query, description = "Should include paid bookings", example = "true"),
    ),
    security(("bearer_auth" = []))
)]
#[get("/booking")]
pub async fn get_own_bookings_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Query<GetOwnBookingsInput>,
) -> impl Responder {
    process_request_secured(
        req,
        &[Role::User, Role::Admin],
        &state,
        input.into_inner(),
        get_own_bookings,
        StatusCode::OK,
    )
    .await
}
