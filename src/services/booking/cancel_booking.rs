use actix_web::http::StatusCode;
use sea_orm::{sqlx::types::chrono::Utc, ActiveModelTrait, EntityTrait, IntoActiveModel};

use crate::{
    api::{
        booking::cancel_booking::{CancelBookingInput, CancelBookingOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::booking,
    util::require_some,
};

pub async fn cancel_booking_service(
    app_state: &AppState,
    input: CancelBookingInput,
) -> Result<CancelBookingOutput, ErrorResponse> {
    let booking = find_booking(app_state, input).await?;
    check_not_payed(&booking)?;
    check_before_start_date(&booking)?;
    set_status_to_canceled(app_state, booking).await
}

async fn find_booking(
    app_state: &AppState,
    input: CancelBookingInput,
) -> Result<booking::Model, ErrorResponse> {
    let booking_option = booking::Entity::find_by_id(input.booking_id)
        .one(app_state.db.as_ref())
        .await?;

    require_some(
        booking_option,
        || format!("Booking with id '{}' not found", input.booking_id),
        StatusCode::NOT_FOUND,
    )
}

fn check_not_payed(booking: &booking::Model) -> Result<(), ErrorResponse> {
    match booking.status {
        booking::BookingStatus::Unpaid => Ok(()),
        booking::BookingStatus::Paid => Err(ErrorResponse::new(
            "Booking is paid".to_owned(),
            StatusCode::BAD_REQUEST,
        )),
        booking::BookingStatus::Canceled => Err(ErrorResponse::new(
            "Booking already canceld".to_owned(),
            StatusCode::BAD_REQUEST,
        )),
    }
}

fn check_before_start_date(booking: &booking::Model) -> Result<(), ErrorResponse> {
    let today = Utc::now();

    if today.date_naive() < booking.start_date {
        Ok(())
    } else {
        Err(ErrorResponse::new(
            "Can't cancel: start date is before today".to_owned(),
            StatusCode::BAD_REQUEST,
        ))
    }
}

async fn set_status_to_canceled(
    app_state: &AppState,
    booking: booking::Model,
) -> Result<CancelBookingOutput, ErrorResponse> {
    booking::ActiveModel {
        status: sea_orm::ActiveValue::Set(booking::BookingStatus::Canceled),
        ..booking.into_active_model()
    }
    .update(app_state.db.as_ref())
    .await?;

    Ok(CancelBookingOutput)
}
