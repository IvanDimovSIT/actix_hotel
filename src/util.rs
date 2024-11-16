use std::{error::Error, future::Future};

use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use jsonwebtoken::get_current_timestamp;
use log::error;
use serde::Serialize;

use crate::{
    api::error_response::ErrorResponse, app_state::AppState, persistence::user::find_user_by_email,
    security::Claims,
};

/// A macro that handles the processing and logging of requests
///
/// Input:
/// - `$state`: The application state (AppState)
/// - `$input`: The input data (needs to implement Validate)
/// - `$service`: The service function that processes request (Fn(&AppState, &$input) -> Result<PromoteOutput, ErrorResponse>)
/// - `$status`: The HTTP response code (StatusCode)
#[macro_export]
macro_rules! process_request {
    ($state:expr, $input:expr, $service:expr, $status:expr) => {{
        use crate::util::serialize_output;
        use log::{error, info};
        let _: &crate::AppState = $state;
        const OPERATION_NAME: &str = stringify!($service);
        info!("Start {}", OPERATION_NAME);
        let validation_result = $input.validate(&$state.validator);
        match validation_result {
            Ok(_) => {
                let output = $service($state, $input).await;
                if output.is_ok() {
                    info!("End success {}", OPERATION_NAME);
                } else {
                    error!("End error {}", OPERATION_NAME);
                }
                serialize_output(output, $status)
            }
            Err(err) => {
                error!("End validation error {}", OPERATION_NAME);
                err.into()
            }
        }
    }};
}

pub fn error_to_response(err: Box<dyn Error>) -> ErrorResponse {
    ErrorResponse::new(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn serialize_output<T>(
    output: Result<T, ErrorResponse>,
    status: StatusCode,
) -> HttpResponse<BoxBody>
where
    T: Serialize,
{
    match output {
        Ok(ok) => serialize_to_http_response(&ok, status),
        Err(err) => serialize_to_http_response(&err, err.status),
    }
}

pub fn serialize_to_http_response<T>(body: &T, status: StatusCode) -> HttpResponse<BoxBody>
where
    T: Serialize,
{
    let result = serde_json::to_string(body);
    if let Err(err) = result {
        error!("Serialization error: {}", err);
        return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::plaintext())
            .body(format!("Error serializing output: {}", err));
    }

    HttpResponse::build(status)
        .content_type(ContentType::json())
        .body(BoxBody::new(result.unwrap()))
}

pub fn create_token_from_user(
    user: &crate::persistence::user::Model,
    app_state: &AppState,
) -> Result<String, ErrorResponse> {
    let exp = get_current_timestamp() + app_state.security_info.jwt_validity;
    let claims = Claims {
        user_id: user.id,
        role: user.role.clone(),
        exp,
    };

    let token = claims.to_token(app_state);
    if let Err(err) = token {
        Err(error_to_response(err))
    } else {
        Ok(token.unwrap())
    }
}

pub async fn find_user(
    app_state: &AppState,
    user_email: &str,
) -> Result<crate::persistence::user::Model, ErrorResponse> {
    let result_find_user = find_user_by_email(&app_state.db, user_email).await;
    if result_find_user.is_err() {
        return Err(ErrorResponse::new(
            "Error fetching data".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let option_find_user = result_find_user.unwrap();
    let user = require_some(
        option_find_user,
        || format!("Email '{}' not found", user_email),
        StatusCode::NOT_FOUND,
    )?;

    Ok(user)
}

pub fn require_some<T, F>(
    option: Option<T>,
    message_provider: F,
    status: StatusCode,
) -> Result<T, ErrorResponse>
where
    F: FnOnce() -> String,
{
    if let Some(some) = option {
        Ok(some)
    } else {
        Err(ErrorResponse::new(message_provider(), status))
    }
}
