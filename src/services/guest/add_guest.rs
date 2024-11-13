use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use log::error;
use sea_orm::{ActiveModelTrait, ActiveValue};
use uuid::Uuid;

use crate::{
    api::guest::add_guest::{AddGuestInput, AddGuestOutput},
    app_state::AppState,
    persistence::{
        guest::{self, find_first_by_ucn_or_card_number_or_phone, ActiveModel},
        handle_db_error,
    },
    services::{error_response, serialize_output},
};

const INVALID_STATE: &str = "Invalid state when searching for existing ucn or id card number";

async fn find_existing_guest(
    app_state: &AppState,
    ucn: &Option<String>,
    id_card_number: &Option<String>,
    phone_number: &Option<String>,
) -> Result<Option<guest::Model>, HttpResponse<BoxBody>> {
    let result = find_first_by_ucn_or_card_number_or_phone(
        app_state.db.as_ref(),
        ucn,
        id_card_number,
        phone_number,
    )
    .await;

    if let Err(err) = result {
        return Err(handle_db_error(err));
    }

    Ok(result.unwrap())
}

fn find_conflicting_fields(
    guest: guest::Model,
    ucn: Option<String>,
    id_card_number: Option<String>,
    phone_number: Option<String>,
) -> HttpResponse<BoxBody> {
    let found_ucn = guest.ucn;
    let found_card_number = guest.id_card_number;
    let found_phone_number = guest.phone_number;
    if found_ucn.is_some() && ucn.is_some() && found_ucn.unwrap() == ucn.clone().unwrap() {
        return error_response("UCN is already in use".to_string(), StatusCode::BAD_REQUEST);
    }

    if found_card_number.is_some()
        && id_card_number.is_some()
        && found_card_number.unwrap() == id_card_number.clone().unwrap()
    {
        return error_response(
            "Id card number is already in use".to_string(),
            StatusCode::BAD_REQUEST,
        );
    }

    if found_phone_number.is_some()
        && phone_number.is_some()
        && found_phone_number.unwrap() == phone_number.clone().unwrap()
    {
        return error_response(
            "Phone number is already in use".to_string(),
            StatusCode::BAD_REQUEST,
        );
    }

    error!("{}", INVALID_STATE);
    error_response(INVALID_STATE.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
}

async fn check_ucn_and_card_number_not_in_use(
    app_state: &AppState,
    input: &AddGuestInput,
) -> Result<(), HttpResponse<BoxBody>> {
    let (ucn, id_card_number) = if let Some(card) = &input.id_card {
        (Some(card.ucn.clone()), Some(card.id_card_number.clone()))
    } else {
        (None, None)
    };

    let find_guest_result =
        find_existing_guest(app_state, &ucn, &id_card_number, &input.phone_number).await?;

    if let Some(guest) = find_guest_result {
        Err(find_conflicting_fields(
            guest,
            ucn,
            id_card_number,
            input.phone_number.clone(),
        ))
    } else {
        Ok(())
    }
}

async fn save_guest(
    app_state: &AppState,
    input: &AddGuestInput,
) -> Result<Uuid, HttpResponse<BoxBody>> {
    let (ucn, id_card_number, id_card_issue_authority, id_card_issue_date, id_card_validity) =
        if let Some(card) = &input.id_card {
            (
                Some(card.ucn.clone()),
                Some(card.id_card_number.clone()),
                Some(card.issue_authority.clone()),
                Some(card.issue_date),
                Some(card.validity),
            )
        } else {
            (None, None, None, None, None)
        };

    let id = Uuid::new_v4();
    let guest = ActiveModel {
        id: ActiveValue::Set(id),
        first_name: ActiveValue::Set(input.first_name.clone()),
        last_name: ActiveValue::Set(input.last_name.clone()),
        date_of_birth: ActiveValue::Set(input.date_of_birth),
        ucn: ActiveValue::Set(ucn),
        id_card_number: ActiveValue::Set(id_card_number),
        id_card_issue_authority: ActiveValue::Set(id_card_issue_authority),
        id_card_issue_date: ActiveValue::Set(id_card_issue_date),
        id_card_validity: ActiveValue::Set(id_card_validity),
        phone_number: ActiveValue::Set(input.phone_number.clone()),
    };
    if let Err(err) = guest.insert(app_state.db.as_ref()).await {
        return Err(handle_db_error(err));
    }

    Ok(id)
}

pub async fn add_guest(app_state: &AppState, input: &AddGuestInput) -> HttpResponse<BoxBody> {
    if let Err(err) = check_ucn_and_card_number_not_in_use(app_state, input).await {
        return err;
    }

    let save_guest_result = save_guest(app_state, input).await;
    if let Err(err) = save_guest_result {
        return err;
    }

    let output = AddGuestOutput {
        guest_id: save_guest_result.unwrap(),
    };
    serialize_output(&output, StatusCode::CREATED)
}
