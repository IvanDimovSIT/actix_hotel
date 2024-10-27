use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use jsonwebtoken::get_current_timestamp;

use crate::{
    api::login::{LoginInput, LoginOutput},
    app_state::AppState,
    persistence::user::{find_user_by_email, Model},
    security::{passwords_match, Claims},
};

use super::serialize_output;

const INVALID_CREDENTIALS: &str = "Invalid credentials";

async fn find_user(
    app_state: &AppState,
    input: &LoginInput,
) -> Result<Model, HttpResponse<BoxBody>> {
    let result_find_user = find_user_by_email(&app_state.db, &input.email).await;
    if result_find_user.is_err() {
        return Err(HttpResponse::Unauthorized().body(INVALID_CREDENTIALS));
    }

    let option_find_user = result_find_user.unwrap();
    if option_find_user.is_none() {
        return Err(HttpResponse::Unauthorized().body(INVALID_CREDENTIALS));
    }
    let user = option_find_user.unwrap();

    Ok(user)
}

fn create_token(user: &Model, app_state: &AppState) -> Result<String, HttpResponse<BoxBody>> {
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

pub async fn login(app_state: &AppState, input: &LoginInput) -> HttpResponse<BoxBody> {
    let result_find_user = find_user(app_state, input).await;
    if let Err(err) = result_find_user {
        return err;
    }

    let user = result_find_user.unwrap();
    if !passwords_match(&input.password, &user.salt, &user.password) {
        return HttpResponse::Unauthorized().body(INVALID_CREDENTIALS);
    }

    let token = create_token(&user, app_state);
    if let Err(err) = token {
        return err;
    }

    let output = LoginOutput {
        token: token.unwrap(),
    };

    serialize_output(&output, StatusCode::OK)
}
