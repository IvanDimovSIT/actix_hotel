use actix_web::{
    get,
    http::StatusCode,
    post, put,
    web::{Data, Json, ServiceConfig},
    HttpRequest, Responder,
};
use utoipa::OpenApi;

use crate::{
    api::{
        auth::{
            change_password::{ChangePasswordInput, ChangePasswordOutput},
            login::{LoginInput, LoginOutput},
            promote::{PromoteInput, PromoteOutput},
            refresh_token::{RefreshTokenInput, RefreshTokenOutput},
            register_user::{RegisterUserInput, RegisterUserOutput},
            reset_password::{ResetPasswordInput, ResetPasswordOutput},
            send_otp::{SendOtpInput, SendOtpOutput},
        },
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::user::Role,
    security::Claims,
    services::auth::{
        change_password::change_password, login::login, promote::promote,
        refresh_token::refresh_token, register_user::register_user, reset_password::reset_password,
        send_otp::send_otp,
    },
    util::{process_request, process_request_secured},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        register_controller,
        login_controller,
        refresh_token_controller,
        change_password_controller,
        send_otp_controller,
        reset_password_controller
    ),
    components(schemas(
        ErrorResponse,
        Role,
        Claims,
        RegisterUserInput,
        RegisterUserOutput,
        LoginInput,
        LoginOutput,
        PromoteInput,
        PromoteOutput,
        RefreshTokenInput,
        RefreshTokenOutput,
        ChangePasswordInput,
        ChangePasswordOutput,
        SendOtpInput,
        SendOtpOutput,
        ResetPasswordInput,
        ResetPasswordOutput
    ))
)]
pub struct AuthApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login_controller);
    cfg.service(register_controller);
    cfg.service(promote_controller);
    cfg.service(refresh_token_controller);
    cfg.service(change_password_controller);
    cfg.service(send_otp_controller);
    cfg.service(reset_password_controller);
}

#[utoipa::path(
    responses(
        (status = 201, description = "Successful Registration", body = RegisterUserOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    ),
    request_body(
        content = RegisterUserInput,
        description = "Registration data",
        content_type = "application/json"
    ),
)]
#[post("/auth/register")]
pub async fn register_controller(
    state: Data<AppState>,
    input: Json<RegisterUserInput>,
) -> impl Responder {
    process_request(
        &state,
        input.into_inner(),
        register_user,
        StatusCode::CREATED,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully logged in", body = LoginOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse)
    ),
    request_body(
        content = LoginInput,
        description = "Login data",
        content_type = "application/json"
    )
)]
#[post("/auth/login")]
pub async fn login_controller(state: Data<AppState>, input: Json<LoginInput>) -> impl Responder {
    process_request(&state, input.into_inner(), login, StatusCode::OK).await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successful Promotion", body = PromoteOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid credentials", body = ErrorResponse),
    ),
    request_body(
        content = PromoteInput,
        description = "Promote data",
        content_type = "application/json"
    ),
    security(("bearer_auth" = []))
)]
#[put("/auth/promote")]
pub async fn promote_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Json<PromoteInput>,
) -> impl Responder {
    process_request_secured(
        req,
        &[Role::Admin],
        &state,
        input.into_inner(),
        promote,
        StatusCode::OK,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successful Promotion", body = RefreshTokenOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
#[get("/auth/refresh")]
pub async fn refresh_token_controller(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let input = RefreshTokenInput {
        ..Default::default()
    };

    process_request_secured(
        req,
        &[Role::User, Role::Admin],
        &state,
        input,
        refresh_token,
        StatusCode::OK,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully changed password", body = ChangePasswordOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid credentials", body = ErrorResponse),
    ),
    request_body(
        content = ChangePasswordInput,
        description = "Change password data",
        content_type = "application/json"
    ),
    security(("bearer_auth" = []))
)]
#[put("/auth/change-password")]
pub async fn change_password_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Json<ChangePasswordInput>,
) -> impl Responder {
    process_request_secured(
        req,
        &[Role::Admin, Role::User],
        &state,
        input.into_inner(),
        change_password,
        StatusCode::OK,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully changed password", body = ChangePasswordOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 404, description = "Invalid email", body = ErrorResponse),
    ),
    request_body(
        content = SendOtpInput,
        description = "User email",
        content_type = "application/json"
    )
)]
#[post("/auth/send-otp")]
pub async fn send_otp_controller(
    state: Data<AppState>,
    input: Json<SendOtpInput>,
) -> impl Responder {
    process_request(&state, input.into_inner(), send_otp, StatusCode::OK).await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully reset password", body = ResetPasswordOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 404, description = "Invalid email", body = ErrorResponse),
    ),
    request_body(
        content = ResetPasswordInput,
        description = "Reset password data",
        content_type = "application/json"
    )
)]
#[post("/auth/reset-password")]
pub async fn reset_password_controller(
    state: Data<AppState>,
    input: Json<ResetPasswordInput>,
) -> impl Responder {
    process_request(&state, input.into_inner(), reset_password, StatusCode::OK).await
}
