use actix_web::http::StatusCode;
use log::error;

use crate::{
    api::error_response::ErrorResponse,
    app_state::AppState,
    persistence::guest::{self, find_first_by_ucn_or_card_number_or_phone},
};

pub mod add_guest;
pub mod find_guest;
pub mod get_guest;
pub mod update_guest;

const INVALID_STATE: &str = "Invalid state when searching for existing ucn or id card number";

async fn find_existing_guest(
    app_state: &AppState,
    ucn: &Option<String>,
    id_card_number: &Option<String>,
    phone_number: &Option<String>,
) -> Result<Option<guest::Model>, ErrorResponse> {
    let guest = find_first_by_ucn_or_card_number_or_phone(
        app_state.db.as_ref(),
        ucn,
        id_card_number,
        phone_number,
    )
    .await?;

    Ok(guest)
}

fn find_conflicting_fields(
    guest: guest::Model,
    ucn: Option<String>,
    id_card_number: Option<String>,
    phone_number: Option<String>,
) -> ErrorResponse {
    let found_ucn = guest.ucn;
    let found_card_number = guest.id_card_number;
    let found_phone_number = guest.phone_number;
    if found_ucn.is_some() && ucn.is_some() && found_ucn.unwrap() == ucn.clone().unwrap() {
        return ErrorResponse::new("UCN is already in use".to_string(), StatusCode::BAD_REQUEST);
    }

    if found_card_number.is_some()
        && id_card_number.is_some()
        && found_card_number.unwrap() == id_card_number.clone().unwrap()
    {
        return ErrorResponse::new(
            "Id card number is already in use".to_string(),
            StatusCode::BAD_REQUEST,
        );
    }

    if found_phone_number.is_some()
        && phone_number.is_some()
        && found_phone_number.unwrap() == phone_number.clone().unwrap()
    {
        return ErrorResponse::new(
            "Phone number is already in use".to_string(),
            StatusCode::BAD_REQUEST,
        );
    }

    error!("{}", INVALID_STATE);
    ErrorResponse::new(INVALID_STATE.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
}
