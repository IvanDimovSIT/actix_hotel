use actix_web::http::StatusCode;

use crate::{
    api::{
        auth::refresh_token::{RefreshTokenInput, RefreshTokenOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::user::{find_user_by_id, Model},
    util::{create_token_from_user, require_some},
};

async fn find_user(
    app_state: &AppState,
    input: &RefreshTokenInput,
) -> Result<Model, ErrorResponse> {
    let option_find_user = find_user_by_id(app_state.db.as_ref(), &input.claims.user_id).await?;

    let user = require_some(
        option_find_user,
        || format!("User with email '{}' not found", &input.claims.user_id),
        StatusCode::NOT_FOUND,
    )?;

    Ok(user)
}

pub async fn refresh_token(
    app_state: &AppState,
    input: &RefreshTokenInput,
) -> Result<RefreshTokenOutput, ErrorResponse> {
    let user = find_user(app_state, input).await?;

    let token = create_token_from_user(&user, app_state)?;
    Ok(RefreshTokenOutput { token })
}
