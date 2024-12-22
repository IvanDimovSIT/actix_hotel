use actix_web::http::StatusCode;

use crate::{
    api::{
        auth::logout::{LogoutInput, LogoutOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::invalidated_token,
    util::{error_to_response, require_some},
};

pub async fn logout(
    app_state: &AppState,
    input: LogoutInput,
) -> Result<LogoutOutput, ErrorResponse> {
    let claims = require_some(
        input.claims,
        || "No claims found for request".to_owned(),
        StatusCode::INTERNAL_SERVER_ERROR,
    )?;

    let token = claims.to_token(app_state);
    if let Err(err) = token {
        return Err(error_to_response(err));
    }

    invalidated_token::invalidate(app_state.db.as_ref(), &token.unwrap()).await?;

    Ok(LogoutOutput)
}
