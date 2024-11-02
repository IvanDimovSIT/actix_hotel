use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};

use crate::{
    api::refresh_token::{RefreshTokenInput, RefreshTokenOutput},
    app_state::AppState,
    persistence::user::{find_user_by_id, Model},
    services::serialize_output,
    util::create_token_from_user,
};

async fn find_user(
    app_state: &AppState,
    input: &RefreshTokenInput,
) -> Result<Model, HttpResponse<BoxBody>> {
    let result_find_user = find_user_by_id(&app_state.db, &input.claims.user_id).await;
    if let Err(err) = result_find_user {
        return Err(HttpResponse::from_error(err));
    }
    let option_find_user = result_find_user.unwrap();

    if option_find_user.is_none() {
        return Err(HttpResponse::NotFound().body(format!(
            "User with email '{}' not found",
            &input.claims.user_id
        )));
    }

    let user = option_find_user.unwrap();

    Ok(user)
}

pub async fn refresh_token(
    app_state: &AppState,
    input: &RefreshTokenInput,
) -> HttpResponse<BoxBody> {
    let find_user_result = find_user(app_state, input).await;
    if let Err(err) = find_user_result {
        return err;
    }
    let user = find_user_result.unwrap();

    let token_result = create_token_from_user(&user, app_state);
    if let Err(err) = token_result {
        return err;
    }

    let token = token_result.unwrap();
    let output = RefreshTokenOutput { token };

    serialize_output(&output, StatusCode::OK)
}
