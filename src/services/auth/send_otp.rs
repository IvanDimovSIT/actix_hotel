use std::time::Duration;
use sea_orm::{sqlx::types::chrono::Utc, ActiveModelTrait};
use uuid::Uuid;

use crate::{
    api::{
        auth::send_otp::{SendOtpInput, SendOtpOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::{
        one_time_password::{self},
        user::{self},
    },
    security::generate_otp,
    util::find_user,
};

async fn create_otp(app_state: &AppState, user: &user::Model) -> Result<String, ErrorResponse> {
    one_time_password::delete_all_for_user(app_state.db.as_ref(), &user.id).await?;

    let otp_code = generate_otp();
    let validity = Utc::now() + Duration::new(app_state.security_info.otp_validity, 0);
    let otp = one_time_password::ActiveModel {
        id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
        user_id: sea_orm::ActiveValue::Set(user.id),
        otp_code: sea_orm::ActiveValue::Set(otp_code.clone()),
        validity: sea_orm::ActiveValue::Set(validity.naive_utc()),
    };
    otp.insert(app_state.db.as_ref()).await?;

    Ok(otp_code)
}

async fn send_email(
    app_state: &AppState,
    user: &user::Model,
    otp_code: &str,
) -> Result<(), ErrorResponse> {
    let body = format!("Password reset code: '{otp_code}'");
    app_state
        .email_service
        .send_text_mail(
            user.email.to_string(),
            "Reset password code".to_string(),
            body,
        )
        .await?;

    Ok(())
}

pub async fn send_otp(
    app_state: &AppState,
    input: SendOtpInput,
) -> Result<SendOtpOutput, ErrorResponse> {
    let user = find_user(app_state, &input.email).await?;
    let otp_code = create_otp(app_state, &user).await?;
    send_email(app_state, &user, &otp_code).await?;

    Ok(SendOtpOutput)
}
