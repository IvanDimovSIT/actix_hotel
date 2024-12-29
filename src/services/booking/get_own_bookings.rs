use actix_web::http::StatusCode;

use crate::{
    api::{
        booking::get_own_bookings::{GetOwnBookingsInput, GetOwnBookingsOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::booking,
    util::require_some,
};

pub async fn get_own_bookings_service(
    app_state: &AppState,
    input: GetOwnBookingsInput,
) -> Result<GetOwnBookingsOutput, ErrorResponse> {
    let user_id = require_some(
        input.user_id,
        || "User id not provided".to_owned(),
        StatusCode::INTERNAL_SERVER_ERROR,
    )?;

    let ids = booking::get_bookings_for_user(
        app_state.db.as_ref(),
        user_id,
        input.include_canceled,
        input.include_paid,
    )
    .await?;

    Ok(GetOwnBookingsOutput { ids })
}
