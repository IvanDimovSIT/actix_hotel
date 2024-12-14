use actix_web::http::StatusCode;
use sea_orm::{
    sqlx::types::chrono::Utc, ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel,
};

use crate::{
    api::{
        booking::pay_booking::{PayBookingInput, PayBookingOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::booking::{self, BookingStatus},
    util::require_some,
};

async fn find_booking(
    app_state: &AppState,
    input: PayBookingInput,
) -> Result<booking::Model, ErrorResponse> {
    let booking = booking::Entity::find_by_id(input.booking_id)
        .one(app_state.db.as_ref())
        .await?;

    require_some(
        booking,
        || format!("Booking with id '{}' not found", input.booking_id),
        StatusCode::NOT_FOUND,
    )
}

async fn set_status_as_paid(
    app_state: &AppState,
    booking: booking::Model,
) -> Result<PayBookingOutput, ErrorResponse> {
    match booking.status {
        booking::BookingStatus::Unpaid => {
            let payment_time = Some(Utc::now().naive_utc());
            let updated_booking = booking::ActiveModel {
                status: ActiveValue::Set(BookingStatus::Paid),
                payment_time: ActiveValue::Set(payment_time),
                ..booking.into_active_model()
            };
            updated_booking.update(app_state.db.as_ref()).await?;

            Ok(PayBookingOutput)
        }
        booking::BookingStatus::Paid => Err(ErrorResponse::new(
            "Booking already paid".to_owned(),
            StatusCode::BAD_REQUEST,
        )),
        booking::BookingStatus::Canceled => Err(ErrorResponse::new(
            "Can't pay, booking is canceled".to_owned(),
            StatusCode::BAD_REQUEST,
        )),
    }
}

pub async fn pay_booking(
    app_state: &AppState,
    input: PayBookingInput,
) -> Result<PayBookingOutput, ErrorResponse> {
    let booking = find_booking(app_state, input).await?;
    set_status_as_paid(app_state, booking).await
}
