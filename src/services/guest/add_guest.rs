use sea_orm::{ActiveModelTrait, ActiveValue};
use uuid::Uuid;

use crate::{
    api::{
        error_response::ErrorResponse,
        guest::add_guest::{AddGuestInput, AddGuestOutput},
    },
    app_state::AppState,
    persistence::guest::ActiveModel,
};

use super::{find_conflicting_fields, find_existing_guest};

pub async fn add_guest_service(
    app_state: &AppState,
    input: AddGuestInput,
) -> Result<AddGuestOutput, ErrorResponse> {
    check_ucn_and_card_number_not_in_use(app_state, &input).await?;

    let guest_id = save_guest(app_state, &input).await?;

    Ok(AddGuestOutput { guest_id })
}

async fn check_ucn_and_card_number_not_in_use(
    app_state: &AppState,
    input: &AddGuestInput,
) -> Result<(), ErrorResponse> {
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

async fn save_guest(app_state: &AppState, input: &AddGuestInput) -> Result<Uuid, ErrorResponse> {
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
    guest.insert(app_state.db.as_ref()).await?;

    Ok(id)
}
