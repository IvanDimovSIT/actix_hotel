use actix_web::http::StatusCode;
use sea_orm::{
    sqlx::types::chrono::Utc, ActiveModelTrait, ActiveValue, DatabaseTransaction, IntoActiveModel,
    TransactionTrait,
};

use crate::{
    api::{
        auth::reset_password::{ResetPasswordInput, ResetPasswordOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::{
        one_time_password::{self, delete_all_for_user, find_otp_and_user_for_user_email},
        user,
    },
    security::hash_password,
    util::require_some,
};

pub async fn reset_password_service(
    app_state: &AppState,
    input: ResetPasswordInput,
) -> Result<ResetPasswordOutput, ErrorResponse> {
    let (otp, user) = find_user_with_otp(app_state, &input).await?;
    validate_otp(&otp, &input)?;

    let transaction = app_state.db.begin().await?;
    change_user_password(&transaction, user, &input).await?;
    delete_all_for_user(&transaction, &otp.user_id).await?;
    transaction.commit().await?;

    Ok(ResetPasswordOutput)
}

pub async fn find_user_with_otp(
    app_state: &AppState,
    input: &ResetPasswordInput,
) -> Result<(one_time_password::Model, user::Model), ErrorResponse> {
    let result = find_otp_and_user_for_user_email(app_state.db.as_ref(), &input.email).await?;

    let (otp, user_option) = require_some(
        result,
        || format!("Not found for email '{}'", input.email),
        StatusCode::NOT_FOUND,
    )?;

    let user = require_some(
        user_option,
        || format!("Not found for email '{}'", input.email),
        StatusCode::NOT_FOUND,
    )?;

    Ok((otp, user))
}

fn validate_otp(
    otp: &one_time_password::Model,
    input: &ResetPasswordInput,
) -> Result<(), ErrorResponse> {
    let is_valid = otp.otp_code == input.otp && otp.validity.time() >= Utc::now().time();

    if !is_valid {
        Err(ErrorResponse::new(
            "Invalid OTP".to_string(),
            StatusCode::BAD_REQUEST,
        ))
    } else {
        Ok(())
    }
}

async fn change_user_password(
    transaction: &DatabaseTransaction,
    user: user::Model,
    input: &ResetPasswordInput,
) -> Result<(), ErrorResponse> {
    let password_hash = hash_password(&input.new_password);
    let mut active_user = user.into_active_model();
    active_user.password = ActiveValue::Set(password_hash);
    active_user.save(transaction).await?;

    Ok(())
}
