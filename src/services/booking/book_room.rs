use std::iter::once;

use actix_web::http::StatusCode;
use sea_orm::{
    sqlx::types::chrono::Utc, ActiveModelTrait, DatabaseTransaction, EntityTrait, IntoActiveModel,
    TransactionTrait,
};
use uuid::Uuid;

use crate::{
    api::{
        booking::book_room::{BookRoomInput, BookRoomOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::{
        booking, booking_guest, guest, room,
        user::{self, Role},
    },
    util::require_some,
    validation::Validator,
};

const MIN_BOOKING_AGE: u32 = 18;

async fn get_booked_by(app_state: &AppState, input: &BookRoomInput) -> Result<Uuid, ErrorResponse> {
    if let Some(admin_id) = &input.booked_by {
        let admin_option = user::find_user_by_id(app_state.db.as_ref(), admin_id).await?;
        let admin = require_some(
            admin_option,
            || format!("Admin with id '{}' not found", admin_id),
            StatusCode::NOT_FOUND,
        )?;
        if admin.role != Role::Admin {
            return Err(ErrorResponse::new(
                "Not booked by admin".to_owned(),
                StatusCode::FORBIDDEN,
            ));
        }

        Ok(admin.id)
    } else {
        Err(ErrorResponse::new(
            "Admin not found".to_owned(),
            StatusCode::NOT_FOUND,
        ))
    }
}

fn check_guest_duplicated(input: &BookRoomInput) -> Result<(), ErrorResponse> {
    let is_main_guest_duplicated = input
        .other_guests
        .iter()
        .any(|guest_id| *guest_id == input.main_guest);

    if is_main_guest_duplicated {
        Err(ErrorResponse::new(
            "Main guest is duplicated".to_owned(),
            StatusCode::BAD_REQUEST,
        ))
    } else {
        Ok(())
    }
}

async fn validate_main_guest(
    app_state: &AppState,
    input: &BookRoomInput,
) -> Result<(), ErrorResponse> {
    let main_guest_option = guest::Entity::find_by_id(input.main_guest)
        .one(app_state.db.as_ref())
        .await?;
    let main_guest = require_some(
        main_guest_option,
        || format!("Main guest with id '{}' not found", input.main_guest),
        StatusCode::NOT_FOUND,
    )?;

    Validator::validate_option(
        &main_guest.id_card_issue_authority,
        "Main guest's id card issue authority",
    )?;
    Validator::validate_option(
        &main_guest.id_card_issue_date,
        "Main guest's id card issue date",
    )?;
    Validator::validate_option(&main_guest.id_card_number, "Main guest's id card number")?;
    Validator::validate_option(
        &main_guest.id_card_validity,
        "Main guest's id card validity",
    )?;
    Validator::validate_option(&main_guest.phone_number, "Main guest's phone number")?;
    Validator::validate_option(&main_guest.ucn, "Main guest's ucn")?;

    let main_guest_age = require_some(
        Utc::now()
            .date_naive()
            .years_since(main_guest.date_of_birth),
        || "Invalid main guest age".to_owned(),
        StatusCode::BAD_REQUEST,
    )?;
    if main_guest_age < MIN_BOOKING_AGE {
        Err(ErrorResponse::new(
            format!("Main guest needs to be at least {MIN_BOOKING_AGE} years old"),
            StatusCode::BAD_REQUEST,
        ))
    } else {
        Ok(())
    }
}

async fn validate_other_guests(
    app_state: &AppState,
    input: &BookRoomInput,
) -> Result<(), ErrorResponse> {
    for guest_id in &input.other_guests {
        let guest = guest::Entity::find_by_id(*guest_id)
            .one(app_state.db.as_ref())
            .await?;
        require_some(
            guest,
            || format!("Guest with id '{}' not found", input.main_guest),
            StatusCode::NOT_FOUND,
        )?;
    }

    Ok(())
}

async fn validate_guests(app_state: &AppState, input: &BookRoomInput) -> Result<(), ErrorResponse> {
    check_guest_duplicated(input)?;
    validate_main_guest(app_state, input).await?;
    validate_other_guests(app_state, input).await?;

    Ok(())
}

async fn find_room_capacity_and_price(
    app_state: &AppState,
    input: &BookRoomInput,
) -> Result<(i16, i64), ErrorResponse> {
    let room_option = room::find_room_by_id(app_state.db.as_ref(), input.room_id).await?;
    let (room, beds) = require_some(
        room_option,
        || format!("Room with id '{}'", input.room_id),
        StatusCode::NOT_FOUND,
    )?;
    if room.is_deleted {
        return Err(ErrorResponse::new(
            format!("Room with id '{}'", input.room_id),
            StatusCode::NOT_FOUND,
        ));
    }

    let capacity = beds.iter().map(|bed| bed.total_capacity).sum();
    let price = room.price;

    Ok((capacity, price))
}

fn check_is_capacity_enough(
    input: &BookRoomInput,
    room_capacity: i16,
) -> Result<(), ErrorResponse> {
    let number_of_guests = (input.other_guests.len() + 1) as i16;
    if room_capacity >= number_of_guests {
        Ok(())
    } else {
        Err(ErrorResponse::new(
            format!(
                "Room capacity '{}' is less than the number of guests '{}'",
                room_capacity, number_of_guests
            ),
            StatusCode::BAD_REQUEST,
        ))
    }
}

async fn check_room_occipied(
    app_state: &AppState,
    input: &BookRoomInput,
) -> Result<(), ErrorResponse> {
    let is_occupied = booking::is_room_occupied_for_period(
        app_state.db.as_ref(),
        input.room_id,
        input.start_date,
        input.end_date,
    )
    .await?;

    if is_occupied {
        Err(ErrorResponse::new(
            "Room is occupied of period".to_owned(),
            StatusCode::BAD_REQUEST,
        ))
    } else {
        Ok(())
    }
}

async fn validate_guest_user(
    app_state: &AppState,
    input: &BookRoomInput,
) -> Result<(), ErrorResponse> {
    if let Some(user_id) = &input.guest_user_id {
        let user = user::find_user_by_id(app_state.db.as_ref(), &user_id).await?;
        require_some(
            user,
            || format!("Guest user with id '{}' not found ", user_id),
            StatusCode::NOT_FOUND,
        )?;
    }

    Ok(())
}

async fn insert_booking(
    transaction: &DatabaseTransaction,
    input: &BookRoomInput,
    room_price: i64,
    admin_id: Uuid,
) -> Result<Uuid, ErrorResponse> {
    let total_price = room_price * ((input.end_date - input.start_date).num_days() + 1);
    let booking = booking::Model {
        id: Uuid::new_v4(),
        main_guest_id: input.main_guest,
        room_id: input.room_id,
        admin_id,
        user_id: input.guest_user_id,
        booking_time: Utc::now().naive_utc(),
        start_date: input.start_date,
        end_date: input.end_date,
        total_price,
        status: booking::BookingStatus::Unpaid,
    }
    .into_active_model()
    .insert(transaction)
    .await?;

    Ok(booking.id)
}

async fn insert_guests(
    transaction: &DatabaseTransaction,
    input: &BookRoomInput,
    booking_id: Uuid,
) -> Result<(), ErrorResponse> {
    for guest_id in input.other_guests.iter().chain(once(&input.main_guest)) {
        booking_guest::Model {
            guest_id: *guest_id,
            booking_id,
        }
        .into_active_model()
        .insert(transaction)
        .await?;
    }

    Ok(())
}

async fn create_booking(
    app_state: &AppState,
    input: BookRoomInput,
    room_price: i64,
) -> Result<BookRoomOutput, ErrorResponse> {
    let admin_id = get_booked_by(app_state, &input).await?;
    let transaction = app_state.db.begin().await?;
    let booking_id = insert_booking(&transaction, &input, room_price, admin_id).await?;
    insert_guests(&transaction, &input, booking_id).await?;
    transaction.commit().await?;

    Ok(BookRoomOutput { booking_id })
}

pub async fn book_room(
    app_state: &AppState,
    input: BookRoomInput,
) -> Result<BookRoomOutput, ErrorResponse> {
    validate_guests(app_state, &input).await?;
    let (room_capacity, room_price) = find_room_capacity_and_price(app_state, &input).await?;
    check_is_capacity_enough(&input, room_capacity)?;
    validate_guest_user(app_state, &input).await?;
    check_room_occipied(app_state, &input).await?;
    create_booking(app_state, input, room_price).await
}
