use actix_web::{body::BoxBody, HttpResponse};
use jsonwebtoken::get_current_timestamp;

use crate::{app_state::AppState, security::Claims};

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
