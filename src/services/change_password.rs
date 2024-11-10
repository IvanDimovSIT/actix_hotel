use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};

use crate::{
    api::change_password::{ChangePasswordInput, ChangePasswordOutput},
    app_state::AppState,
    persistence::{
        handle_db_error,
        user::{self, find_user_by_id},
    },
    security::{hash_password, passwords_match},
    util::require_some,
};

use super::{error_response, error_to_response, serialize_output};

async fn find_user(
    app_state: &AppState,
    input: &ChangePasswordInput,
) -> Result<user::Model, HttpResponse<BoxBody>> {
    let find_user_result = find_user_by_id(app_state.db.as_ref(), &input.user_id).await;
    if let Err(err) = find_user_result {
        return Err(error_to_response(err));
    }
    let option_user = find_user_result.unwrap();

    require_some(
        option_user,
        || "User not found".to_string(),
        StatusCode::NOT_FOUND,
    )
}

async fn save_new_user(
    app_state: &AppState,
    user: user::Model,
    input: &ChangePasswordInput,
) -> Result<(), HttpResponse<BoxBody>> {
    let new_password = hash_password(&input.new_password);

    let mut active_user = user.into_active_model();
    active_user.password = ActiveValue::set(new_password);
    let result = active_user.update(app_state.db.as_ref()).await;
    if let Err(err) = result {
        return Err(handle_db_error(err));
    }

    Ok(())
}

pub async fn change_password(
    app_state: &AppState,
    input: &ChangePasswordInput,
) -> HttpResponse<BoxBody> {
    let result_find_user = find_user(app_state, input).await;
    if let Err(err) = result_find_user {
        return err;
    }
    let user = result_find_user.unwrap();
    if !passwords_match(&input.old_password, &user.password) {
        return error_response("Invalid credentials".to_string(), StatusCode::UNAUTHORIZED);
    }

    if let Err(err) = save_new_user(app_state, user, input).await {
        return err;
    }

    serialize_output(&ChangePasswordOutput, StatusCode::OK)
}
