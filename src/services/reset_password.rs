use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use sea_orm::{
    sqlx::types::chrono::Utc, ActiveModelTrait, ActiveValue, DatabaseTransaction, IntoActiveModel,
    TransactionTrait,
};

use crate::{
    api::reset_password::{ResetPasswordInput, ResetPasswordOutput},
    app_state::AppState,
    persistence::{
        handle_db_error,
        one_time_password::{self, delete_all_for_user, find_otp_and_user_for_user_email},
        user,
    },
    security::hash_password,
    services::{error_response, error_to_response},
    util::require_some,
};

use super::serialize_output;

pub async fn find_user_with_otp(
    app_state: &AppState,
    input: &ResetPasswordInput,
) -> Result<(one_time_password::Model, user::Model), HttpResponse<BoxBody>> {
    let result = find_otp_and_user_for_user_email(app_state.db.as_ref(), &input.email).await;
    if let Err(err) = result {
        return Err(error_to_response(err));
    }
    let option = require_some(
        result.unwrap(),
        || format!("Not found for email '{}'", input.email),
        StatusCode::NOT_FOUND,
    );
    if let Err(err) = option {
        return Err(err);
    }

    let (otp, user_option) = option.unwrap();
    let user = require_some(
        user_option,
        || format!("Not found for email '{}'", input.email),
        StatusCode::NOT_FOUND,
    );

    Ok((otp, user.unwrap()))
}

fn validate_otp(
    otp: &one_time_password::Model,
    input: &ResetPasswordInput,
) -> Result<(), HttpResponse<BoxBody>> {
    let is_valid = otp.otp_code == input.otp && otp.validity.time() >= Utc::now().time();

    if !is_valid {
        Err(error_response(
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
) -> Result<(), HttpResponse<BoxBody>> {
    let password_hash = hash_password(&input.new_password);
    let mut active_user = user.into_active_model();
    active_user.password = ActiveValue::Set(password_hash);

    let save_result = active_user.save(transaction).await;
    if let Err(err) = save_result {
        return Err(handle_db_error(err));
    }

    Ok(())
}

pub async fn reset_password(
    app_state: &AppState,
    input: &ResetPasswordInput,
) -> HttpResponse<BoxBody> {
    let find_user_result = find_user_with_otp(app_state, &input).await;
    if let Err(err) = find_user_result {
        return err;
    }
    let (otp, user) = find_user_result.unwrap();
    if let Err(err) = validate_otp(&otp, input) {
        return err;
    }

    let transaction_result = app_state.db.begin().await;
    if let Err(err) = transaction_result {
        return handle_db_error(err);
    }

    let transaction = transaction_result.unwrap();

    let save_result = change_user_password(&transaction, user, input).await;
    if let Err(err) = save_result {
        return err;
    }

    if let Err(err) = delete_all_for_user(&transaction, &otp.user_id).await {
        return error_to_response(err);
    }

    if let Err(err) = transaction.commit().await {
        return handle_db_error(err);
    }

    serialize_output(&ResetPasswordOutput, StatusCode::OK)
}
