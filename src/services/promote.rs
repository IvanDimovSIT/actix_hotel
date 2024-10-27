use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};

use crate::{
    api::promote::{PromoteInput, PromoteOutput},
    app_state::AppState,
    persistence::{
        handle_db_error,
        user::{find_user_by_email, Model, Role},
    },
};

use super::serialize_output;

async fn find_user(
    app_state: &AppState,
    input: &PromoteInput,
) -> Result<Model, HttpResponse<BoxBody>> {
    let result_find_user = find_user_by_email(&app_state.db, &input.email).await;
    if let Err(err) = result_find_user {
        return Err(HttpResponse::from_error(err));
    }

    let option_find_user = result_find_user.unwrap();
    if option_find_user.is_none() {
        return Err(
            HttpResponse::NotFound().body(format!("User with email '{}' not found", input.email))
        );
    }
    let user = option_find_user.unwrap();

    Ok(user)
}

pub async fn promote(app_state: &AppState, input: &PromoteInput) -> HttpResponse<BoxBody> {
    let found_user = find_user(app_state, input).await;
    if let Err(err) = found_user {
        return err;
    }
    let mut user = found_user.unwrap().into_active_model();
    user.role = ActiveValue::Set(Role::Admin);

    let result = user.save(app_state.db.as_ref()).await;
    if let Err(err) = result {
        return handle_db_error(err);
    }

    serialize_output(&PromoteOutput, StatusCode::OK)
}
