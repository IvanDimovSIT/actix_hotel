use std::time::Duration;

use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use sea_orm::{sqlx::types::chrono::Utc, ActiveModelTrait};
use uuid::Uuid;

use crate::{
    api::auth::send_otp::{SendOtpInput, SendOtpOutput},
    app_state::AppState,
    persistence::{
        handle_db_error,
        one_time_password::{self},
        user::{self},
    },
    security::generate_otp,
    services::{error_to_response, serialize_output},
    util::find_user,
};

async fn create_otp(app_state: &AppState, user: &user::Model) -> Result<String, HttpResponse> {
    if let Err(err) = one_time_password::delete_all_for_user(app_state.db.as_ref(), &user.id).await
    {
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

async fn send_email(
    app_state: &AppState,
    user: &user::Model,
    otp_code: &str,
) -> Result<(), HttpResponse<BoxBody>> {
    let body = format!("Password reset code: '{otp_code}'");
    let send_email_result = app_state
        .email_service
        .send_text_mail(
            user.email.to_string(),
            "Reset password code".to_string(),
            body,
        )
        .await;

    if let Err(err) = send_email_result {
        return Err(error_to_response(err));
    }

    Ok(())
}

pub async fn send_otp(app_state: &AppState, input: &SendOtpInput) -> HttpResponse<BoxBody> {
    let find_user_result = find_user(app_state, &input.email).await;
    if let Err(err) = find_user_result {
        return err;
    }
    let user = find_user_result.unwrap();

    let otp_result = create_otp(app_state, &user).await;
    if let Err(err) = otp_result {
        return err;
    }
    let otp_code = otp_result.unwrap();

    let send_mail_result = send_email(app_state, &user, &otp_code).await;
    if let Err(err) = send_mail_result {
        return err;
    }

    serialize_output(&SendOtpOutput, StatusCode::OK)
}
