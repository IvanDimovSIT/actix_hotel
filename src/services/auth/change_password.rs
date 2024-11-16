use actix_web::http::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};

use crate::{
    api::{
        auth::change_password::{ChangePasswordInput, ChangePasswordOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::user::{self, find_user_by_id},
    security::{hash_password, passwords_match},
    util::require_some,
};

async fn find_user(
    app_state: &AppState,
    input: &ChangePasswordInput,
) -> Result<user::Model, ErrorResponse> {
    let option_user = find_user_by_id(app_state.db.as_ref(), &input.user_id).await?;

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
) -> Result<(), ErrorResponse> {
    let new_password = hash_password(&input.new_password);

    let mut active_user = user.into_active_model();
    active_user.password = ActiveValue::set(new_password);
    active_user.update(app_state.db.as_ref()).await?;

    Ok(())
}

pub async fn change_password(
    app_state: &AppState,
    input: &ChangePasswordInput,
) -> Result<ChangePasswordOutput, ErrorResponse> {
    let user = find_user(app_state, input).await?;

    if !passwords_match(&input.old_password, &user.password) {
        return Err(ErrorResponse::new(
            "Invalid credentials".to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    save_new_user(app_state, user, input).await?;

    Ok(ChangePasswordOutput)
}
