use actix_web::{
    get, post, put,
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
        error_response::ErrorReponse,
    },
    app_state::AppState,
    persistence::user::Role,
    security::{decode_claims, Claims},
    services::auth::{
        change_password::change_password, login::login, promote::promote,
        refresh_token::refresh_token, register_user::register_user, reset_password::reset_password,
        send_otp::send_otp,
    },
    validation::Validate,
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
        ErrorReponse,
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
        (status = 400, description = "Invalid input", body = ErrorReponse)
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
    let register_user_input = input.into_inner();
    if let Err(err) = register_user_input.validate(&state.validator) {
        return err;
    }

    register_user(&state.db, &register_user_input).await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully logged in", body = LoginOutput),
        (status = 400, description = "Invalid input", body = ErrorReponse),
        (status = 401, description = "Invalid credentials", body = ErrorReponse)
    ),
    request_body(
        content = LoginInput,
        description = "Login data",
        content_type = "application/json"
    )
)]
#[post("/auth/login")]
pub async fn login_controller(state: Data<AppState>, input: Json<LoginInput>) -> impl Responder {
    let login_input = input.into_inner();
    if let Err(err) = login_input.validate(&state.validator) {
        return err;
    }

    login(&state, &login_input).await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successful Promotion", body = PromoteOutput),
        (status = 400, description = "Invalid input", body = ErrorReponse),
        (status = 401, description = "Invalid credentials", body = ErrorReponse),
        (status = 403, description = "Invalid credentials", body = ErrorReponse),
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
    let authorization = decode_claims(&req, &state, &[Role::Admin]);
    if let Err(err) = authorization {
        return err;
    }

    let promote_input = input.into_inner();
    if let Err(err) = promote_input.validate(&state.validator) {
        return err;
    }

    promote(&state, &promote_input).await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successful Promotion", body = RefreshTokenOutput),
        (status = 400, description = "Invalid input", body = ErrorReponse),
        (status = 401, description = "Invalid credentials", body = ErrorReponse),
        (status = 404, description = "User not found", body = ErrorReponse),
    ),
    security(("bearer_auth" = []))
)]
#[get("/auth/refresh")]
pub async fn refresh_token_controller(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let authorization = decode_claims(&req, &state, &[Role::User, Role::Admin]);
    if let Err(err) = authorization {
        return err;
    }

    let refresh_token_input = RefreshTokenInput {
        claims: authorization.unwrap(),
    };

    refresh_token(&state, &refresh_token_input).await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully changed password", body = ChangePasswordOutput),
        (status = 400, description = "Invalid input", body = ErrorReponse),
        (status = 401, description = "Invalid credentials", body = ErrorReponse),
        (status = 403, description = "Invalid credentials", body = ErrorReponse),
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
    let authorization = decode_claims(&req, &state, &[Role::Admin, Role::User]);
    if let Err(err) = authorization {
        return err;
    }
    let user_id = authorization.unwrap().user_id;

    let change_password_input = input.into_inner();
    let change_password_input = ChangePasswordInput {
        user_id,
        ..change_password_input
    };
    if let Err(err) = change_password_input.validate(&state.validator) {
        return err;
    }

    change_password(&state, &change_password_input).await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully changed password", body = ChangePasswordOutput),
        (status = 400, description = "Invalid input", body = ErrorReponse),
        (status = 404, description = "Invalid email", body = ErrorReponse),
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
    let send_otp_input = input.into_inner();
    if let Err(err) = send_otp_input.validate(&state.validator) {
        return err;
    }

    send_otp(&state, &send_otp_input).await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully reset password", body = ResetPasswordOutput),
        (status = 400, description = "Invalid input", body = ErrorReponse),
        (status = 404, description = "Invalid email", body = ErrorReponse),
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
    let reset_password_input = input.into_inner();
    if let Err(err) = reset_password_input.validate(&state.validator) {
        return err;
    }

    reset_password(&state, &reset_password_input).await
}
