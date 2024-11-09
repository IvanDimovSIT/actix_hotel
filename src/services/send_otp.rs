use std::time::Duration;

use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use log::error;
use sea_orm::{sqlx::types::chrono::Utc, ActiveModelTrait};
use uuid::Uuid;

use crate::{
    api::send_otp::{SendOtpInput, SendOtpOutput},
    app_state::AppState,
    persistence::{
        handle_db_error,
        one_time_password::{self},
        user::{self, find_user_by_email},
    },
    security::generate_otp,
    util::require_some,
};

use super::{error_response, error_to_response, serialize_output};

async fn find_user(
    app_state: &AppState,
    input: &SendOtpInput,
) -> Result<crate::persistence::user::Model, HttpResponse<BoxBody>> {
    let result_find_user = find_user_by_email(&app_state.db, &input.email).await;
    if result_find_user.is_err() {
        return Err(error_response(
            "Error fetching data".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let option_find_user = result_find_user.unwrap();
    let user = require_some(
        option_find_user,
        || format!("Email '{}' not found", input.email),
        StatusCode::NOT_FOUND,
    )?;

    Ok(user)
}

async fn create_otp(app_state: &AppState, user: &user::Model) -> Result<String, HttpResponse> {
    if let Err(err) = one_time_password::delete_all_for_user(&app_state.db, &user.id).await {
        return Err(error_to_response(err));
    }

    let otp_code = generate_otp();
    let validity = Utc::now() + Duration::new(app_state.security_info.otp_validity, 0);
    let otp = one_time_password::ActiveModel {
        id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
        user_id: sea_orm::ActiveValue::Set(user.id),
        otp_code: sea_orm::ActiveValue::Set(otp_code.clone()),
        validity: sea_orm::ActiveValue::Set(validity.naive_utc()),
    };

    if let Err(err) = otp.insert(app_state.db.as_ref()).await {
        return Err(handle_db_error(err));
    }

    Ok(otp_code)
}

pub async fn send_otp(app_state: &AppState, input: &SendOtpInput) -> HttpResponse<BoxBody> {
    let find_user_result = find_user(app_state, input).await;
    if let Err(err) = find_user_result {
        return err;
    }
    let user = find_user_result.unwrap();

    let otp_result = create_otp(app_state, &user).await;
    if let Err(err) = otp_result {
        return err;
    }
    let otp_code = otp_result.unwrap();

    //TODO: Send otp email for user
    error!("Sending emails not implemented! OTP:'{otp_code}'");

    serialize_output(&SendOtpOutput, StatusCode::OK)
}