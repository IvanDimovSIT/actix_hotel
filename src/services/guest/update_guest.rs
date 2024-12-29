use actix_web::http::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

use crate::{
    api::{
        error_response::ErrorResponse,
        guest::update_guest::{UpdateGuestInput, UpdateGuestOutput},
    },
    app_state::AppState,
    persistence,
    services::guest::find_existing_guest,
    util::require_some,
};

use super::find_conflicting_fields;

pub async fn update_guest_service(
    app_state: &AppState,
    input: UpdateGuestInput,
) -> Result<UpdateGuestOutput, ErrorResponse> {
    check_exists(app_state, &input).await?;
    check_ucn_and_card_number_not_in_use(app_state, &input).await?;
    save_guest(app_state, input).await
}

async fn check_exists(app_state: &AppState, input: &UpdateGuestInput) -> Result<(), ErrorResponse> {
    require_some(
        persistence::guest::Entity::find_by_id(input.id.unwrap())
            .one(app_state.db.as_ref())
            .await?,
        || format!("Guest with id '{}' not found", input.id.unwrap()),
        StatusCode::NOT_FOUND,
    )?;

    Ok(())
}

async fn check_ucn_and_card_number_not_in_use(
    app_state: &AppState,
    input: &UpdateGuestInput,
) -> Result<(), ErrorResponse> {
    let (ucn, id_card_number) = if let Some(card) = &input.id_card {
        (Some(card.ucn.clone()), Some(card.id_card_number.clone()))
    } else {
        (None, None)
    };

    let find_guest_result =
        find_existing_guest(app_state, &ucn, &id_card_number, &input.phone_number).await?;

    match find_guest_result {
        Some(found) => {
            if found.id != input.id.unwrap() {
                Err(find_conflicting_fields(
                    found,
                    ucn,
                    id_card_number,
                    input.phone_number.clone(),
                ))
            } else {
                Ok(())
            }
        }
        None => Ok(()),
    }
}

async fn save_guest(
    app_state: &AppState,
    input: UpdateGuestInput,
) -> Result<UpdateGuestOutput, ErrorResponse> {
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

    let guest = persistence::guest::ActiveModel {
        id: ActiveValue::Unchanged(input.id.unwrap()),
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
    guest.save(app_state.db.as_ref()).await?;

    Ok(UpdateGuestOutput)
}
