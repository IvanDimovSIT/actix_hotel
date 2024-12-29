use actix_web::http::StatusCode;

use crate::{
    api::{
        auth::login::{LoginInput, LoginOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::user::{find_user_by_email, Model},
    security::passwords_match,
    util::{create_token_from_user, require_some},
};

const INVALID_CREDENTIALS: &str = "Invalid credentials";

pub async fn login_service(
    app_state: &AppState,
    input: LoginInput,
) -> Result<LoginOutput, ErrorResponse> {
    let user = find_user(app_state, &input).await?;

    if !passwords_match(&input.password, &user.password) {
        return Err(ErrorResponse::new(
            INVALID_CREDENTIALS.to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    let token = create_token_from_user(&user, app_state)?;

    Ok(LoginOutput { token })
}

async fn find_user(app_state: &AppState, input: &LoginInput) -> Result<Model, ErrorResponse> {
    let result_find_user = find_user_by_email(&app_state.db, &input.email).await;
    if result_find_user.is_err() {
        return Err(ErrorResponse::new(
            INVALID_CREDENTIALS.to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    let option_find_user = result_find_user.unwrap();
    let user = require_some(
        option_find_user,
        || INVALID_CREDENTIALS.to_string(),
        StatusCode::UNAUTHORIZED,
    )?;

    Ok(user)
}
