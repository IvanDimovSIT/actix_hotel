use crate::{
    api::{
        error_response::ErrorResponse,
        guest::find_guest::{FindGuestInput, FindGuestOutput},
    },
    app_state::AppState,
    persistence::guest,
};

pub async fn find_guest_service(
    app_state: &AppState,
    input: FindGuestInput,
) -> Result<FindGuestOutput, ErrorResponse> {
    let guest_ids = guest::find_all_ids_by_criteria(
        app_state.db.as_ref(),
        input.first_name,
        input.last_name,
        input.phone_number,
        input.date_of_birth,
        input.ucn,
    )
    .await?;

    Ok(FindGuestOutput { guest_ids })
}
