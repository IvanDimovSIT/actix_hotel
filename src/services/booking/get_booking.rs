use actix_web::http::StatusCode;
use sea_orm::{EntityTrait, ModelTrait};

use crate::{
    api::{
        booking::get_booking::{BookingGuest, GetBookingInput, GetBookingOutput},
        error_response::ErrorResponse,
        guest::GuestIdCard,
    },
    app_state::AppState,
    persistence::{self, booking, guest},
    util::require_some,
};

async fn find_booking(
    app_state: &AppState,
    input: &GetBookingInput,
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

fn check_has_access(
    booking: &booking::Model,
    input: &GetBookingInput,
) -> Result<(), ErrorResponse> {
    let role = require_some(
        input.role.clone(),
        || "Role not provided".to_owned(),
        StatusCode::FORBIDDEN,
    )?;
    match role {
        crate::persistence::user::Role::User => match (&booking.user_id, &input.user_id) {
            (Some(id1), Some(id2)) => {
                if id1 == id2 {
                    Ok(())
                } else {
                    Err(ErrorResponse::new(
                        "Access not allowed".to_owned(),
                        StatusCode::FORBIDDEN,
                    ))
                }
            }
            _ => Err(ErrorResponse::new(
                "Access not allowed".to_owned(),
                StatusCode::FORBIDDEN,
            )),
        },
        crate::persistence::user::Role::Admin => Ok(()),
    }
}

fn convert_id_card(guest: &guest::Model) -> Option<GuestIdCard> {
    match (
        &guest.id_card_issue_authority,
        &guest.id_card_issue_date,
        &guest.id_card_validity,
        &guest.id_card_number,
        &guest.ucn,
    ) {
        (
            Some(issue_authority),
            Some(issue_date),
            Some(validity),
            Some(id_card_number),
            Some(ucn),
        ) => Some(GuestIdCard {
            ucn: ucn.clone(),
            id_card_number: id_card_number.clone(),
            issue_authority: issue_authority.clone(),
            issue_date: issue_date.clone(),
            validity: validity.clone(),
        }),
        _ => None,
    }
}

fn convert_guest(guest: Option<guest::Model>) -> Result<BookingGuest, ErrorResponse> {
    let some_guest = require_some(
        guest,
        || "Error fetching guests".to_owned(),
        StatusCode::INTERNAL_SERVER_ERROR,
    )?;

    Ok(BookingGuest {
        id_card: convert_id_card(&some_guest),
        first_name: some_guest.first_name,
        last_name: some_guest.last_name,
        date_of_birth: some_guest.date_of_birth,
        phone_number: some_guest.phone_number,
    })
}

async fn convert_to_output(
    app_state: &AppState,
    booking: booking::Model,
) -> Result<GetBookingOutput, ErrorResponse> {
    let booking_guests = booking
        .find_related(persistence::booking_guest::Entity)
        .all(app_state.db.as_ref())
        .await?;

    let main_guest_option = guest::Entity::find_by_id(booking.main_guest_id)
        .one(app_state.db.as_ref())
        .await?;
    let main_guest = convert_guest(main_guest_option)?;

    let guests_futures: Vec<_> = booking_guests
        .into_iter()
        .map(|bg| {
            bg.find_related(persistence::guest::Entity)
                .one(app_state.db.as_ref())
        })
        .collect();

    let mut other_guests = Vec::with_capacity(guests_futures.len());
    for guest in guests_futures {
        other_guests.push(convert_guest(guest.await?)?);
    }

    Ok(GetBookingOutput {
        main_guest,
        other_guests,
        room_id: booking.room_id,
        admin_id: booking.admin_id,
        user_id: booking.user_id,
        booking_time: booking.booking_time,
        payment_time: booking.payment_time,
        start_date: booking.start_date,
        end_date: booking.end_date,
        total_price: booking.total_price,
        status: booking.status,
    })
}

pub async fn get_booking(
    app_state: &AppState,
    input: GetBookingInput,
) -> Result<GetBookingOutput, ErrorResponse> {
    let booking = find_booking(app_state, &input).await?;
    check_has_access(&booking, &input)?;

    convert_to_output(app_state, booking).await
}
