use actix_web::http::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue};
use uuid::Uuid;

use crate::{
    api::{
        auth::register_user::{RegisterUserInput, RegisterUserOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::user::{self, find_user_by_email},
    security::hash_password,
};

pub async fn register_user_service(
    app_state: &AppState,
    input: RegisterUserInput,
) -> Result<RegisterUserOutput, ErrorResponse> {
    let find_user_result = find_user_by_email(&app_state.db, &input.email).await?;

    if find_user_result.is_some() {
        return Err(ErrorResponse::new(
            format!("Email {} already taken", &input.email),
            StatusCode::BAD_REQUEST,
        ));
    }

    let password = hash_password(&input.password);

    let user_to_save = user::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        email: ActiveValue::Set(input.email.clone()),
        password: ActiveValue::Set(password),
        role: ActiveValue::Set(user::Role::User),
    };

    let user = user_to_save.insert(app_state.db.as_ref()).await?;

    Ok(RegisterUserOutput { user_id: user.id })
}
