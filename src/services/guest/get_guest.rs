use actix_web::http::StatusCode;
use log::error;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::{
    api::{
        error_response::ErrorResponse,
        guest::{
            get_guest::{GetGuestInput, GetGuestOutput},
            GuestIdCard,
        },
    },
    app_state::AppState,
    persistence::guest,
    util::require_some,
};

async fn find_guest_in_db(
    app_state: &AppState,
    input: GetGuestInput,
) -> Result<guest::Model, ErrorResponse> {
    let result = guest::Entity::find_by_id(input.guest_id)
        .one(app_state.db.as_ref())
        .await?;

    require_some(
        result,
        || format!("Guest with id:'{}' not found", input.guest_id),
        StatusCode::NOT_FOUND,
    )
}

fn id_card_error(guest_id: Uuid) -> impl Fn() -> String {
    move || {
        error!(
            "Unexpected error getting id card for guest id '{}'",
            guest_id
        );
        "Error finding id card information".to_string()
    }
}

fn require_id_card_data<T>(option: Option<T>, guest_id: Uuid) -> Result<T, ErrorResponse> {
    require_some(
        option,
        id_card_error(guest_id),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

fn convert_model_to_output(guest: guest::Model) -> Result<GetGuestOutput, ErrorResponse> {
    let id_card = if guest.id_card_number.is_some() {
        let id_card_number = guest.id_card_number.unwrap();
        let ucn = require_id_card_data(guest.ucn, guest.id)?;
        let issue_authority = require_id_card_data(guest.id_card_issue_authority, guest.id)?;
        let issue_date = require_id_card_data(guest.id_card_issue_date, guest.id)?;
        let validity = require_id_card_data(guest.id_card_validity, guest.id)?;

        Some(GuestIdCard {
            ucn,
            id_card_number,
            issue_authority,
            issue_date,
            validity,
        })
    } else {
        None
    };

    Ok(GetGuestOutput {
        first_name: guest.first_name,
        last_name: guest.last_name,
        date_of_birth: guest.date_of_birth,
        id_card,
        phone_number: guest.phone_number,
    })
}

pub async fn get_guest(
    app_state: &AppState,
    input: GetGuestInput,
) -> Result<GetGuestOutput, ErrorResponse> {
    let guest = find_guest_in_db(app_state, input).await?;
    Ok(convert_model_to_output(guest)?)
}
