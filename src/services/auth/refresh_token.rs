use actix_web::http::StatusCode;

use crate::{
    api::{
        auth::refresh_token::{RefreshTokenInput, RefreshTokenOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::user::{find_user_by_id, Model},
    security::Claims,
    util::{create_token_from_user, require_some},
};

async fn find_user(app_state: &AppState, claims: &Claims) -> Result<Model, ErrorResponse> {
    let option_find_user = find_user_by_id(app_state.db.as_ref(), &claims.user_id).await?;

    let user = require_some(
        option_find_user,
        || format!("User with email '{}' not found", &claims.user_id),
        StatusCode::NOT_FOUND,
    )?;

    Ok(user)
}

pub async fn refresh_token(
    app_state: &AppState,
    input: RefreshTokenInput,
) -> Result<RefreshTokenOutput, ErrorResponse> {
    let claims = require_some(
        input.claims,
        || "Claims are empty".to_string(),
        StatusCode::INTERNAL_SERVER_ERROR,
    )?;
    let user = find_user(app_state, &claims).await?;

    let token = create_token_from_user(&user, app_state)?;
    Ok(RefreshTokenOutput { token })
}
