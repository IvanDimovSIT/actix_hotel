use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use jsonwebtoken::get_current_timestamp;

use crate::{
    app_state::AppState, persistence::user::find_user_by_email, security::Claims,
    services::error_response,
};

pub fn create_token_from_user(
    user: &crate::persistence::user::Model,
    app_state: &AppState,
) -> Result<String, HttpResponse<BoxBody>> {
    let exp = get_current_timestamp() + app_state.security_info.jwt_validity;
    let claims = Claims {
        user_id: user.id,
        role: user.role.clone(),
        exp,
    };

    let token = claims.to_token(app_state);
    if let Err(err) = token {
        Err(HttpResponse::from_error(err))
    } else {
        Ok(token.unwrap())
    }
}

pub async fn find_user(
    app_state: &AppState,
    user_email: &str,
) -> Result<crate::persistence::user::Model, HttpResponse<BoxBody>> {
    let result_find_user = find_user_by_email(&app_state.db, user_email).await;
    if result_find_user.is_err() {
        return Err(error_response(
            "Error fetching data".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let option_find_user = result_find_user.unwrap();
    let user = require_some(
        option_find_user,
        || format!("Email '{}' not found", user_email),
        StatusCode::NOT_FOUND,
    )?;

    Ok(user)
}

pub fn require_some<T, F>(
    option: Option<T>,
    message_provider: F,
    status: StatusCode,
) -> Result<T, HttpResponse<BoxBody>>
where
    F: FnOnce() -> String,
{
    if let Some(some) = option {
        Ok(some)
    } else {
        Err(error_response(message_provider(), status))
    }
}
